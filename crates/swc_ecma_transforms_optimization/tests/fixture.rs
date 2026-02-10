use std::path::PathBuf;

use std::rc::Rc;

use swc_common::{Mark, comments::SingleThreadedComments, pass::Repeat};
use swc_ecma_ast::Pass;
use swc_ecma_parser::{EsSyntax, Syntax};
use swc_ecma_transforms_base::fixer::paren_remover;
use swc_ecma_transforms_optimization::simplify::{dce::dce, expr_simplifier};
use swc_ecma_transforms_testing::test_fixture;
use swc_ecma_visit::visit_mut_pass;

fn remover(comments: Rc<SingleThreadedComments>) -> impl Pass {
    visit_mut_pass(paren_remover(Some(
        Box::leak(Box::new(comments)) as _
    )))
}

#[testing::fixture("tests/dce/**/input.js")]
fn dce_single_pass(input: PathBuf) {
    let output = input.with_file_name("output.js");

    test_fixture(
        Syntax::Es(EsSyntax {
            decorators: true,
            ..Default::default()
        }),
        &|t| {
            let unresolved_mark = Mark::new();

            (remover(t.comments.clone()), dce(Default::default(), unresolved_mark))
        },
        &input,
        &output,
        Default::default(),
    );
}

#[testing::fixture("tests/dce/**/input.js")]
fn dce_repeated(input: PathBuf) {
    let output = input.with_file_name("output.full.js");

    test_fixture(
        Syntax::Es(EsSyntax {
            decorators: true,
            ..Default::default()
        }),
        &|t| {
            (
                remover(t.comments.clone()),
                Repeat::new(dce(Default::default(), Mark::new())),
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

#[testing::fixture("tests/dce-jsx/**/input.js")]
fn dce_jsx(input: PathBuf) {
    let output = input.with_file_name("output.js");

    test_fixture(
        Syntax::Es(EsSyntax {
            decorators: true,
            jsx: true,
            ..Default::default()
        }),
        &|t| (remover(t.comments.clone()), dce(Default::default(), Mark::new())),
        &input,
        &output,
        Default::default(),
    );
}

#[testing::fixture("tests/expr-simplifier/**/input.js")]
fn expr(input: PathBuf) {
    let output = input.with_file_name("output.js");

    test_fixture(
        Syntax::Es(Default::default()),
        &|t| {
            let top_level_mark = Mark::fresh(Mark::root());

            (
                remover(t.comments.clone()),
                Repeat::new(expr_simplifier(top_level_mark, Default::default())),
            )
        },
        &input,
        &output,
        Default::default(),
    );
}
