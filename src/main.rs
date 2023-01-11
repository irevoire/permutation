// use std::marker::PhantomData;

use permutation::Permutation;

fn main() {
    let mut permutation = Permutation::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    while let Some(perm) = permutation.next() {
        println!("{:?}", perm);
    }
}
