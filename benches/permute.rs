use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use permutation::Permutation;

pub fn permute_small_elements(c: &mut Criterion) {
    // Permute 1 element
    let mut group = c.benchmark_group("permute 1");
    group.bench_function("self", |b| {
        b.iter(|| {
            let mut permutation = Permutation::new([1]);
            while let Some(perm) = permutation.next() {
                black_box(perm);
            }
        })
    });
    group.bench_function("itertools", |b| {
        b.iter(|| {
            let permutation = (1..=1).permutations(1);
            permutation.for_each(|el| drop(black_box(el)));
        })
    });
    group.bench_function("sanzen", |b| {
        b.iter(|| {
            let permutation = aoc_sanzen::permutation::Permutations::from(vec![1]);
            permutation.iter().for_each(|el| drop(black_box(el)));
        })
    });
    group.finish();

    // Permute 5 elements
    let mut group = c.benchmark_group("permute 5");
    group.bench_function("self", |b| {
        b.iter(|| {
            let mut permutation = Permutation::new([0, 1, 2, 3, 4]);
            while let Some(perm) = permutation.next() {
                black_box(perm);
            }
        })
    });
    group.bench_function("itertools", |b| {
        b.iter(|| {
            let mut permutation = (0..5).permutations(5);
            while let Some(perm) = permutation.next() {
                black_box(perm);
            }
        })
    });
    group.bench_function("sanzen", |b| {
        b.iter(|| {
            let permutation = aoc_sanzen::permutation::Permutations::from(vec![0, 1, 2, 3, 4]);
            permutation.iter().for_each(|el| drop(black_box(el)));
        })
    });
    group.finish();

    // Permute 10 elements
    let mut group = c.benchmark_group("permute 10");
    group.bench_function("self", |b| {
        b.iter(|| {
            let mut permutation = Permutation::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
            while let Some(perm) = permutation.next() {
                black_box(perm);
            }
        })
    });
    group.bench_function("itertools", |b| {
        b.iter(|| {
            let mut permutation = (0..10).permutations(10);
            while let Some(perm) = permutation.next() {
                black_box(perm);
            }
        })
    });
    group.bench_function("sanzen", |b| {
        b.iter(|| {
            let permutation =
                aoc_sanzen::permutation::Permutations::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
            permutation.iter().for_each(|el| drop(black_box(el)));
        })
    });
    group.finish();
}

pub fn permute_large_elements(c: &mut Criterion) {
    // 64 bytes; medium size
    let medium_one = [1_u8; 64];
    let medium_two = [2_u8; 64];
    let medium_three = [3_u8; 64];
    let medium_four = [4_u8; 64];
    let medium_five = [5_u8; 64];

    // KiB
    let kib_one = [1_u8; 1024]; // 1KiB
    let kib_two = [2_u8; 1024]; // 1KiB
    let kib_three = [3_u8; 1024]; // 1KiB
    let kib_four = [4_u8; 1024]; // 1KiB
    let kib_five = [5_u8; 1024]; // 1KiB

    // // MiB
    // let mib_one = [1_u8; 1024 * 1024]; // 1MiB
    // let mib_two = [2_u8; 1024 * 1024]; // 1MiB
    // let mib_three = [3_u8; 1024 * 1024]; // 1MiB
    // let mib_four = [4_u8; 1024 * 1024]; // 1MiB
    // let mib_five = [5_u8; 1024 * 1024]; // 1MiB

    // 64 bytes
    let mut group = c.benchmark_group("64 bytes");
    group.bench_function("self", |b| {
        b.iter(|| {
            let mut permutation = Permutation::new([
                medium_one,
                medium_two,
                medium_three,
                medium_four,
                medium_five,
            ]);
            while let Some(perm) = permutation.next() {
                black_box(perm);
            }
        })
    });
    group.bench_function("itertools", |b| {
        b.iter(|| {
            let mut permutation = [
                medium_one,
                medium_two,
                medium_three,
                medium_four,
                medium_five,
            ]
            .into_iter()
            .permutations(5);
            while let Some(perm) = permutation.next() {
                black_box(perm);
            }
        })
    });
    group.bench_function("sanzen", |b| {
        b.iter(|| {
            let permutation = aoc_sanzen::permutation::Permutations::from(vec![
                medium_one,
                medium_two,
                medium_three,
                medium_four,
                medium_five,
            ]);
            permutation.iter().for_each(|el| drop(black_box(el)));
        })
    });
    group.finish();

    // 1KiB
    let mut group = c.benchmark_group("1KiB");
    group.bench_function("self", |b| {
        b.iter(|| {
            let mut permutation =
                Permutation::new([kib_one, kib_two, kib_three, kib_four, kib_five]);
            while let Some(perm) = permutation.next() {
                black_box(perm);
            }
        })
    });
    group.bench_function("itertools", |b| {
        b.iter(|| {
            let mut permutation = [kib_one, kib_two, kib_three, kib_four, kib_five]
                .into_iter()
                .permutations(5);
            while let Some(perm) = permutation.next() {
                black_box(perm);
            }
        })
    });
    group.bench_function("sanzen", |b| {
        b.iter(|| {
            let permutation = aoc_sanzen::permutation::Permutations::from(vec![
                kib_one, kib_two, kib_three, kib_four, kib_five,
            ]);
            permutation.iter().for_each(|el| drop(black_box(el)));
        })
    });
    group.finish();

    /* throw a SIGSEV
    // 1MiB
    let mut group = c.benchmark_group("1MiB");
    group.bench_function("self", |b| {
        b.iter(|| {
            let mut permutation =
                Permutation::new([mib_one, mib_two, mib_three, mib_four, mib_five]);
            while let Some(perm) = permutation.next() {
                black_box(perm);
            }
        })
    });
    group.bench_function("itertools", |b| {
        b.iter(|| {
            let mut permutation = [mib_one, mib_two, mib_three, mib_four, mib_five]
                .into_iter()
                .permutations(5);
            while let Some(perm) = permutation.next() {
                black_box(perm);
            }
        })
    });
    group.bench_function("sanzen", |b| {
        b.iter(|| {
            let permutation = aoc_sanzen::permutation::Permutations::from(vec![
                mib_one, mib_two, mib_three, mib_four, mib_five,
            ]);
            permutation.iter().for_each(|el| drop(black_box(el)));
        })
    });
    group.finish();
    */
}

criterion_group!(benches, permute_small_elements, permute_large_elements);
criterion_main!(benches);
