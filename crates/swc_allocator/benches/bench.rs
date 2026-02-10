extern crate swc_malloc;

use allocator_api2::{boxed::Box as AllocBox, vec::Vec as AllocVec};
use codspeed_criterion_compat::{Bencher, Criterion, black_box, criterion_group, criterion_main};
use swc_allocator::allocators::Arena;

fn bench_alloc(c: &mut Criterion) {
    fn direct_alloc_std(b: &mut Bencher, times: usize) {
        b.iter(|| {
            let mut buf = std::vec::Vec::new();
            for i in 0..times {
                let item: std::boxed::Box<usize> = black_box(std::boxed::Box::new(black_box(i)));
                buf.push(item);
            }
        })
    }

    fn arena_alloc_no_scope(b: &mut Bencher, times: usize) {
        b.iter(|| {
            let arena = Arena::default();
            let mut vec = AllocVec::new_in(&arena);
            for i in 0..times {
                let item: AllocBox<usize, &Arena> =
                    black_box(AllocBox::new_in(black_box(i), &arena));
                vec.push(item);
            }
        })
    }

    fn arena_alloc_scoped(b: &mut Bencher, times: usize) {
        b.iter(|| {
            let arena = Arena::default();
            let mut vec = AllocVec::new_in(&arena);
            for i in 0..times {
                let item: AllocBox<usize, &Arena> =
                    black_box(AllocBox::new_in(black_box(i), &arena));
                vec.push(item);
            }
        })
    }

    c.bench_function("common/allocator/alloc/std/1000000", |b| {
        direct_alloc_std(b, 1000000)
    });
    c.bench_function("common/allocator/alloc/no-scope/1000000", |b| {
        arena_alloc_no_scope(b, 1000000)
    });
    c.bench_function("common/allocator/alloc/scoped/1000000", |b| {
        arena_alloc_scoped(b, 1000000)
    });
}

criterion_group!(benches, bench_alloc);
criterion_main!(benches);
