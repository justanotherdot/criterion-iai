//use criterion::{black_box, criterion_group, criterion_main, Criterion};
use core::sync::atomic::{AtomicBool, Ordering};
use iai::black_box;

fn assign<'a, 'b>(allocator: &'b mut [u64], src: &'a [u64]) -> u64 {
    let mut k = 0;
    for item in src {
        unsafe {
            *allocator.get_unchecked_mut(k) = *item;
            k += 1;
        }
    }
    unsafe { *allocator.get_unchecked(99) }
}

fn store_load<'a, 'b>(allocator: &'b mut [u64], src: &'a [u64]) -> u64 {
    allocator.copy_from_slice(src);
    unsafe { *allocator.get_unchecked(99) }
}

fn get_xs() -> &'static mut [u64; 100] {
    static TAKEN: AtomicBool = AtomicBool::new(false);
    if TAKEN.swap(true, Ordering::AcqRel) {
        panic!();
    }
    static mut XS: [u64; 100] = [0; 100];
    unsafe { &mut XS }
}

fn get_ys() -> &'static [u64; 100] {
    static TAKEN: AtomicBool = AtomicBool::new(false);
    if TAKEN.swap(true, Ordering::AcqRel) {
        panic!();
    }
    static mut YS: [u64; 100] = [1; 100];
    unsafe { &YS }
}

fn iai_assign() {
    let xs = get_xs();
    let ys = get_ys();
    store_load(black_box(xs), black_box(ys));
}

fn iai_store_load() {
    let xs = get_xs();
    let ys = get_ys();
    assign(black_box(xs), black_box(ys));
}

//fn criterion_benchmark(c: &mut Criterion) {
//    let mut xs: [u64; 10000] = [0; 10000];
//    let ys: [u64; 10000] = [1; 10000];
//
//    c.bench_function("store and load", |b| {
//        b.iter(|| store_load(black_box(&mut xs), black_box(&ys)))
//    });
//
//    c.bench_function("assign sequential", |b| {
//        b.iter(|| assign(black_box(&mut xs), black_box(&ys)))
//    });
//}

//criterion_group!(benches, criterion_benchmark);
//criterion_main!(benches);

iai::main!(iai_assign, iai_store_load);
