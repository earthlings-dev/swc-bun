use std::hint::black_box;

use allocator_api2::{alloc::Global, boxed::Box, vec::Vec};
use swc_allocator::allocators::Arena;

#[test]
fn direct_alloc_std() {
    let mut buf = std::vec::Vec::new();
    for i in 0..1000 {
        let item: std::boxed::Box<usize> = black_box(std::boxed::Box::new(black_box(i)));
        buf.push(item);
    }
}

#[test]
fn direct_alloc_no_scope() {
    let arena = Arena::default();
    let mut vec = Vec::new_in(&arena);
    for i in 0..1000 {
        let item: Box<usize, &Arena> = black_box(Box::new_in(black_box(i), &arena));
        vec.push(item);
    }
}

#[test]
fn direct_alloc_in_scope() {
    let arena = Arena::default();
    let mut vec = Vec::new_in(&arena);

    for i in 0..1000 {
        let item: Box<usize, &Arena> = black_box(Box::new_in(black_box(i), &arena));
        vec.push(item);
    }
}

#[test]
fn escape() {
    let arena = Arena::default();
    let obj = Box::new_in(1234, &arena);

    assert_eq!(*obj, 1234);
    drop(obj);
}

#[test]
fn global_allocator_escape() {
    let obj = { Box::new_in(1234, Global) };

    assert_eq!(*obj, 1234);
    // Object created with global allocator should outlive the allocator.
    drop(obj);
}
