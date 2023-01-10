// use std::marker::PhantomData;

use permutation::Permutation;

fn main() {
    // let input = vec![0, 1, 2, 3];

    let mut permutation = Permutation::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    while let Some(perm) = permutation.next() {
        println!("{:?}", perm);
    }
}
