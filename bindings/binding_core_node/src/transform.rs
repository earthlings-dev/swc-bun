use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context as _;
use napi::{
    bindgen_prelude::{AbortSignal, AsyncTask, Buffer},
    Env, Task,
};
use path_clean::clean;
use swc_core::{
    base::{config::Options, Compiler, TransformOutput},
    common::{FileName, Spanned},
    ecma::{ast::Program, visit::VisitMutWith},
    node::{deserialize_json, get_deserialized, MapErr},
};
use tracing::instrument;

use crate::{get_compiler, get_fresh_compiler, span_normalize::SpanDenormalizer, util::try_with};

/// Denormalizes a deserialized Program's 0-based spans back to SourceMap-
/// relative positions.
///
/// When `parseSync` normalizes spans to 0-based byte offsets, the original
/// source file is still registered in the shared `SourceMap`. This function
/// finds that file (by filename + byte length) and shifts all spans to be
/// relative to it so that `process_js` / `lookup_char_pos` can locate the
/// `SourceFile` and produce correct source maps.
///
/// Falls back to registering a dummy file if the original cannot be found
/// (e.g. when the AST was constructed outside of `parseSync`).
fn denormalize_program_spans(
    cm: &swc_core::common::SourceMap,
    program: &mut Program,
    filename: &str,
) {
    let source_len = program.span().hi().0;

    // Look for the original file registered by parseSync.
    let offset = {
        let files = cm.files();
        files.iter().rev().find_map(|f| {
            let file_len = f.end_pos.0 - f.start_pos.0;
            let name_matches = match &*f.name {
                FileName::Real(path) => path.to_string_lossy() == filename,
                FileName::Anon => filename.is_empty(),
                _ => false,
            };
            if name_matches && file_len == source_len {
                Some(f.start_pos.0)
            } else {
                None
            }
        })
    }; // lock dropped

    let offset = offset.unwrap_or_else(|| {
        let fm = cm.new_source_file(
            if filename.is_empty() {
                FileName::Anon.into()
            } else {
                FileName::Real(filename.into()).into()
            },
            " ".repeat(source_len as usize),
        );
        fm.start_pos.0
    });

    program.visit_mut_with(&mut SpanDenormalizer { offset });
}

/// Input to transform
#[derive(Debug)]
pub enum Input {
    /// json string
    Program(String),
    /// Raw source code.
    Source { src: String },
    /// File
    File(PathBuf),
}

pub struct TransformTask {
    pub c: Arc<Compiler>,
    pub input: Input,
    pub options: Buffer,
}

#[napi]
impl Task for TransformTask {
    type JsValue = TransformOutput;
    type Output = TransformOutput;

    #[instrument(level = "trace", skip_all)]
    fn compute(&mut self) -> napi::Result<Self::Output> {
        let mut options: Options = serde_json::from_slice(self.options.as_ref())?;
        if !options.filename.is_empty() {
            options.config.adjust(Path::new(&options.filename));
        }

        let error_format = options.experimental.error_format.unwrap_or_default();

        try_with(
            self.c.cm.clone(),
            !options.config.error.filename.into_bool(),
            error_format,
            |handler| {
                self.c.run(|| match &self.input {
                    Input::Program(ref s) => {
                        let mut program: Program =
                            deserialize_json(s).expect("failed to deserialize Program");
                        denormalize_program_spans(&self.c.cm, &mut program, &options.filename);
                        self.c.process_js(handler, program, &options)
                    }

                    Input::File(ref path) => {
                        let fm = self.c.cm.load_file(path).context("failed to load file")?;
                        self.c.process_js_file(fm, handler, &options)
                    }

                    Input::Source { src } => {
                        let fm = self.c.cm.new_source_file(
                            if options.filename.is_empty() {
                                FileName::Anon.into()
                            } else {
                                FileName::Real(options.filename.clone().into()).into()
                            },
                            src.clone(),
                        );

                        self.c.process_js_file(fm, handler, &options)
                    }
                })
            },
        )
        .convert_err()
    }

    fn resolve(&mut self, _env: Env, result: Self::Output) -> napi::Result<Self::JsValue> {
        Ok(result)
    }
}

#[napi]
#[instrument(level = "trace", skip_all)]
pub fn transform(
    src: String,
    is_module: bool,
    options: Buffer,
    signal: Option<AbortSignal>,
) -> napi::Result<AsyncTask<TransformTask>> {
    crate::util::init_default_trace_subscriber();

    let (input, c) = if is_module {
        (Input::Program(src), get_compiler())
    } else {
        (Input::Source { src }, get_fresh_compiler())
    };

    let task = TransformTask { c, input, options };
    Ok(AsyncTask::with_optional_signal(task, signal))
}

#[napi]
#[instrument(level = "trace", skip_all)]
pub fn transform_sync(s: String, is_module: bool, opts: Buffer) -> napi::Result<TransformOutput> {
    crate::util::init_default_trace_subscriber();

    let c = if is_module {
        get_compiler()
    } else {
        get_fresh_compiler()
    };

    let mut options: Options = get_deserialized(&opts)?;

    if !options.filename.is_empty() {
        options.config.adjust(Path::new(&options.filename));
    }

    let error_format = options.experimental.error_format.unwrap_or_default();

    try_with(
        c.cm.clone(),
        !options.config.error.filename.into_bool(),
        error_format,
        |handler| {
            c.run(|| {
                if is_module {
                    let mut program: Program =
                        deserialize_json(s.as_str()).context("failed to deserialize Program")?;
                    denormalize_program_spans(&c.cm, &mut program, &options.filename);
                    c.process_js(handler, program, &options)
                } else {
                    let fm = c.cm.new_source_file(
                        if options.filename.is_empty() {
                            FileName::Anon.into()
                        } else {
                            FileName::Real(options.filename.clone().into()).into()
                        },
                        s,
                    );
                    c.process_js_file(fm, handler, &options)
                }
            })
        },
    )
    .convert_err()
}

#[napi]
#[instrument(level = "trace", skip_all)]
pub fn transform_file(
    src: String,
    _is_module: bool,
    options: Buffer,
    signal: Option<AbortSignal>,
) -> napi::Result<AsyncTask<TransformTask>> {
    crate::util::init_default_trace_subscriber();

    let c = get_fresh_compiler();

    let path = clean(&src);
    let task = TransformTask {
        c,
        input: Input::File(path),
        options,
    };
    Ok(AsyncTask::with_optional_signal(task, signal))
}

#[napi]
pub fn transform_file_sync(
    s: String,
    is_module: bool,
    opts: Buffer,
) -> napi::Result<TransformOutput> {
    crate::util::init_default_trace_subscriber();

    let c = get_fresh_compiler();

    let mut options: Options = get_deserialized(&opts)?;

    if !options.filename.is_empty() {
        options.config.adjust(Path::new(&options.filename));
    }

    let error_format = options.experimental.error_format.unwrap_or_default();

    try_with(
        c.cm.clone(),
        !options.config.error.filename.into_bool(),
        error_format,
        |handler| {
            c.run(|| {
                if is_module {
                    let mut program: Program =
                        deserialize_json(s.as_str()).context("failed to deserialize Program")?;
                    denormalize_program_spans(&c.cm, &mut program, &options.filename);
                    c.process_js(handler, program, &options)
                } else {
                    let fm = c.cm.load_file(Path::new(&s)).expect("failed to load file");
                    c.process_js_file(fm, handler, &options)
                }
            })
        },
    )
    .convert_err()
}
