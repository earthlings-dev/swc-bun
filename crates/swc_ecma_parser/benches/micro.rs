extern crate swc_malloc;

use codspeed_criterion_compat::{Bencher, Criterion, black_box, criterion_group, criterion_main};
use swc_common::FileName;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

fn bench_expr(b: &mut Bencher, syntax: Syntax, src: &'static str) {
    let _ = ::testing::run_test(false, |cm, _| {
        let fm = cm.new_source_file(FileName::Anon.into(), src);

        b.iter(|| {
            let _ = black_box({
                let lexer = Lexer::new(syntax, Default::default(), StringInput::from(&*fm), None);
                let mut parser = Parser::new_from(lexer);
                parser.parse_expr()
            });
        });

        Ok(())
    });
}

fn bench_micro(c: &mut Criterion) {
    c.bench_function("es/parser/micro/new_expr_ts", |b| {
        bench_expr(b, Syntax::Typescript(TsSyntax::default()), "new Foo()")
    });

    c.bench_function("es/parser/micro/new_expr_es", |b| {
        bench_expr(b, Syntax::Es(Default::default()), "new Foo()")
    });

    c.bench_function("es/parser/micro/member_expr_ts", |b| {
        bench_expr(b, Syntax::Typescript(TsSyntax::default()), "a.b.c.d.e.f")
    });

    c.bench_function("es/parser/micro/member_expr_es", |b| {
        bench_expr(b, Syntax::Es(Default::default()), "a.b.c.d.e.f")
    });
}

criterion_group!(benches, bench_micro);
criterion_main!(benches);
