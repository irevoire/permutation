use iai::black_box;
use itertools::Itertools;
use permutation::Permutation;

fn myself() {
    let mut permutation = Permutation::new([0, 1, 2, 3, 4]);
    while let Some(perm) = permutation.next() {
        black_box(perm);
    }
}

fn itertools() {
    let mut permutation = (0..5).permutations(5);
    while let Some(perm) = permutation.next() {
        black_box(perm);
    }
}

fn sanzen() {
    let permutation = aoc_sanzen::permutation::Permutations::from(vec![0, 1, 2, 3, 4]);
    permutation.iter().for_each(|el| drop(black_box(el)));
}

iai::main!(myself, itertools, sanzen);
