use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context as _;
use napi::{
    bindgen_prelude::{AbortSignal, AsyncTask, Buffer},
    Either, Env, Task,
};
use swc_core::{
    base::{
        config::{ErrorFormat, ParseOptions},
        Compiler,
    },
    common::{comments::Comments, BytePos, FileName, Mark, Span},
    ecma::{ast::Program, transforms::base::resolver, visit::VisitMutWith},
    node::{deserialize_json, get_deserialized, MapErr},
};

use crate::{get_compiler, get_fresh_compiler, span_normalize::SpanNormalizer, util::try_with};

/// Sets the Program's top-level span to cover the entire source file
/// `[0, source_byte_length)`. The parser only spans from first token to last
/// token, so trailing whitespace is excluded — this corrects it for consumers
/// that expect 0-based, full-file byte offsets.
fn set_program_span_to_source_len(p: &mut Program, source_len: u32) {
    let full_span = Span::new(BytePos(0), BytePos(source_len));
    match p {
        Program::Module(m) => m.span = full_span,
        Program::Script(s) => s.span = full_span,
    }
}

// ----- Parsing -----

pub struct ParseTask {
    pub c: Arc<Compiler>,
    pub filename: FileName,
    pub src: String,
    pub options: String,
}

pub struct ParseFileTask {
    pub c: Arc<Compiler>,
    pub path: PathBuf,
    pub options: String,
}

#[napi]
impl Task for ParseTask {
    type JsValue = String;
    type Output = String;

    fn compute(&mut self) -> napi::Result<Self::Output> {
        let options: ParseOptions = deserialize_json(&self.options)?;
        let fm = self
            .c
            .cm
            .new_source_file(self.filename.clone().into(), self.src.clone());
        let file_start_pos = fm.start_pos;
        let source_len = fm.end_pos.0 - fm.start_pos.0;

        let comments = if options.comments {
            Some(self.c.comments() as &dyn Comments)
        } else {
            None
        };

        let program = try_with(self.c.cm.clone(), false, ErrorFormat::Normal, |handler| {
            let mut p = self.c.parse_js(
                fm,
                handler,
                options.target,
                options.syntax,
                options.is_module,
                comments,
            )?;

            p.visit_mut_with(&mut resolver(
                Mark::new(),
                Mark::new(),
                options.syntax.typescript(),
            ));

            p.visit_mut_with(&mut SpanNormalizer::new(file_start_pos));
            set_program_span_to_source_len(&mut p, source_len);

            Ok(p)
        })
        .convert_err()?;

        let ast_json = serde_json::to_string(&program)?;

        Ok(ast_json)
    }

    fn resolve(&mut self, _env: Env, result: Self::Output) -> napi::Result<Self::JsValue> {
        Ok(result)
    }
}

#[napi]
impl Task for ParseFileTask {
    type JsValue = String;
    type Output = String;

    fn compute(&mut self) -> napi::Result<Self::Output> {
        let program = try_with(self.c.cm.clone(), false, ErrorFormat::Normal, |handler| {
            self.c.run(|| {
                let options: ParseOptions = deserialize_json(&self.options)?;

                let fm = self
                    .c
                    .cm
                    .load_file(&self.path)
                    .context("failed to read module")?;
                let file_start_pos = fm.start_pos;
                let source_len = fm.end_pos.0 - fm.start_pos.0;

                let c = self.c.comments().clone();
                let comments = if options.comments {
                    Some(&c as &dyn Comments)
                } else {
                    None
                };

                let mut p = self.c.parse_js(
                    fm,
                    handler,
                    options.target,
                    options.syntax,
                    options.is_module,
                    comments,
                )?;

                p.visit_mut_with(&mut resolver(
                    Mark::new(),
                    Mark::new(),
                    options.syntax.typescript(),
                ));

                p.visit_mut_with(&mut SpanNormalizer::new(file_start_pos));
                set_program_span_to_source_len(&mut p, source_len);

                Ok(p)
            })
        })
        .convert_err()?;

        let ast_json = serde_json::to_string(&program)?;

        Ok(ast_json)
    }

    fn resolve(&mut self, _env: Env, result: Self::Output) -> napi::Result<Self::JsValue> {
        Ok(result)
    }
}

fn stringify(src: Either<Buffer, String>) -> String {
    match src {
        Either::A(src) => String::from_utf8_lossy(src.as_ref()).into_owned(),
        Either::B(src) => src,
    }
}

#[napi]
pub fn parse(
    src: Either<Buffer, String>,
    options: Buffer,
    filename: Option<String>,
    signal: Option<AbortSignal>,
) -> AsyncTask<ParseTask> {
    crate::util::init_default_trace_subscriber();

    let c = get_compiler();
    let src = stringify(src);
    let options = String::from_utf8_lossy(options.as_ref()).into_owned();
    let filename = if let Some(value) = filename {
        FileName::Real(value.into())
    } else {
        FileName::Anon
    };

    AsyncTask::with_optional_signal(
        ParseTask {
            c,
            filename,
            src,
            options,
        },
        signal,
    )
}

#[napi]
pub fn parse_sync(
    src: Either<Buffer, String>,
    opts: Buffer,
    filename: Option<String>,
) -> napi::Result<String> {
    crate::util::init_default_trace_subscriber();

    let c = get_compiler();
    let src = stringify(src);
    let options: ParseOptions = get_deserialized(&opts)?;
    let filename = if let Some(value) = filename {
        FileName::Real(value.into())
    } else {
        FileName::Anon
    };

    let program = try_with(c.cm.clone(), false, ErrorFormat::Normal, |handler| {
        c.run(|| {
            let fm = c.cm.new_source_file(filename.into(), src);
            let file_start_pos = fm.start_pos;
            let source_len = fm.end_pos.0 - fm.start_pos.0;

            let comments = if options.comments {
                Some(c.comments() as &dyn Comments)
            } else {
                None
            };

            let mut p = c.parse_js(
                fm,
                handler,
                options.target,
                options.syntax,
                options.is_module,
                comments,
            )?;

            p.visit_mut_with(&mut resolver(
                Mark::new(),
                Mark::new(),
                options.syntax.typescript(),
            ));

            p.visit_mut_with(&mut SpanNormalizer::new(file_start_pos));
            set_program_span_to_source_len(&mut p, source_len);

            Ok(p)
        })
    })
    .convert_err()?;

    Ok(serde_json::to_string(&program)?)
}

#[napi]
pub fn parse_file_sync(path: String, opts: Buffer) -> napi::Result<String> {
    crate::util::init_default_trace_subscriber();
    let c = get_fresh_compiler();
    let options: ParseOptions = get_deserialized(&opts)?;

    let program = {
        try_with(c.cm.clone(), false, ErrorFormat::Normal, |handler| {
            let fm =
                c.cm.load_file(Path::new(path.as_str()))
                    .expect("failed to read program file");
            let file_start_pos = fm.start_pos;
            let source_len = fm.end_pos.0 - fm.start_pos.0;

            let comments = if options.comments {
                Some(c.comments() as &dyn Comments)
            } else {
                None
            };

            let mut p = c.parse_js(
                fm,
                handler,
                options.target,
                options.syntax,
                options.is_module,
                comments,
            )?;
            p.visit_mut_with(&mut resolver(
                Mark::new(),
                Mark::new(),
                options.syntax.typescript(),
            ));

            p.visit_mut_with(&mut SpanNormalizer::new(file_start_pos));
            set_program_span_to_source_len(&mut p, source_len);

            Ok(p)
        })
    }
    .convert_err()?;

    Ok(serde_json::to_string(&program)?)
}

#[napi]
pub fn parse_file(
    path: String,
    options: Buffer,
    signal: Option<AbortSignal>,
) -> AsyncTask<ParseFileTask> {
    crate::util::init_default_trace_subscriber();

    let c = get_fresh_compiler();
    let path = PathBuf::from(&path);
    let options = String::from_utf8_lossy(options.as_ref()).into_owned();

    AsyncTask::with_optional_signal(ParseFileTask { c, path, options }, signal)
}
