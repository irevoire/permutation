//! Steinhaus–Johnson–Trotter algorithm, Even's speedup
//! https://en.wikipedia.org/wiki/Steinhaus%E2%80%93Johnson%E2%80%93Trotter_algorithm
//!
//!
//! The Steinhaus–Johnson–Trotter algorithm or Johnson–Trotter algorithm generates
//! all of the permutations of n elements. Each permutation in the sequence that
//! it generates differs from the previous permutation by swapping two adjacent
//! elements of the sequence.
//!
//! An improvement of the Steinhaus–Johnson–Trotter algorithm by Shimon Even
//! provides an improvement to the running time
//! of the algorithm by storing additional information for each element in the
//! permutation: its position, and a direction (positive, negative, or zero) in which
//! it is currently moving.

#[derive(Default, Debug)]
pub struct Permutation {
    input: Vec<usize>,
    numbers: Vec<usize>,
    directions: Vec<isize>,
    positions: Vec<isize>,
    terminated: bool,
}

impl Permutation {
    /// @param n numbers in the arrray 1..n
    pub fn new(input: Vec<usize>) -> Self {
        let mut this = Self::default();
        // Initially, the direction of the number 1 is zero,
        // and all other elements have a negative direction
        for i in 0..input.len() {
            this.numbers.push(i);
            this.positions.push(i as isize);
            if i == 0 {
                this.directions.push(0);
            } else {
                this.directions.push(-1);
            }
        }

        this.input = input;
        this
    }

    /// Returns the next permutation of the Steinhaus–Johnson–Trotter algorithm.
    ///
    /// Returns None if the permutations have been exhausted.
    pub fn next(&mut self) -> Option<Vec<usize>> {
        if self.terminated {
            return None;
        }
        let copy = self.input.clone();

        if let Some((_, index)) = self
            .directions
            .iter()
            .zip(&self.positions)
            .rev()
            .find(|(dir, _pos)| **dir != 0)
        {
            self.swap_with_next_element_in_direction(*index as usize);
        } else {
            self.terminated = true;
        }
        Some(copy)
    }

    fn swap_with_next_element_in_direction(&mut self, index: usize) {
        // precondition self.directions[index] not 0
        // swaps it in the indicated direction
        let number = self.numbers[index] as usize;
        let new_index = (index as isize + self.directions[number]) as usize;
        let other_number = self.numbers[new_index] as usize;

        self.positions.swap(number, other_number);
        self.numbers.swap(new_index, index);
        self.input.swap(new_index, index);

        // If self causes the chosen element to reach the first or last position
        // within the permutation, or if the next element in the same direction
        // is greater than the chosen element, the direction of the chosen element is set to zero:
        if new_index == 0
            || new_index == self.numbers.len() - 1
            || self.numbers[new_index]
                < self.numbers[(new_index as isize + self.directions[number]) as usize]
        {
            self.directions[number] = 0;
        }

        // After each step, all elements greater than the chosen element
        // (which previously had direction zero) have their directions
        // set to indicate motion toward the chosen element.
        for i in 0..self.numbers.len() {
            if i == new_index {
                continue;
            }
            if self.numbers[i] > self.numbers[new_index] {
                self.directions[self.numbers[i]] = if i < new_index { 1 } else { -1 };
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero() {
        let mut permutation = Permutation::new(vec![]);

        insta::assert_debug_snapshot!(permutation.next(), @r###"
        Some(
            [],
        )
        "###);
        insta::assert_debug_snapshot!(permutation.next(), @"None");
    }

    #[test]
    fn one() {
        let mut permutation = Permutation::new(vec![1]);

        insta::assert_debug_snapshot!(permutation.next(), @r###"
        Some(
            [
                1,
            ],
        )
        "###);
        insta::assert_debug_snapshot!(permutation.next(), @"None");
    }

    #[test]
    fn two() {
        let mut permutation = Permutation::new(vec![1, 2]);

        insta::assert_debug_snapshot!(permutation.next(), @r###"
        Some(
            [
                1,
                2,
            ],
        )
        "###);
        insta::assert_debug_snapshot!(permutation.next(), @r###"
        Some(
            [
                2,
                1,
            ],
        )
        "###);
        insta::assert_debug_snapshot!(permutation.next(), @"None");
    }

    #[test]
    fn three() {
        let mut permutation = Permutation::new(vec![1, 2, 3]);

        let mut ret = String::new();

        while let Some(perm) = permutation.next() {
            ret.push_str(
                &perm
                    .into_iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            );
            ret.push_str("\n");
        }
        insta::assert_display_snapshot!(ret, @r###"
        1, 2, 3
        1, 3, 2
        3, 1, 2
        3, 2, 1
        2, 3, 1
        2, 1, 3
        "###);
    }

    #[test]
    fn four() {
        let mut permutation = Permutation::new(vec![1, 2, 3, 4]);

        let mut ret = String::new();

        while let Some(perm) = permutation.next() {
            ret.push_str(
                &perm
                    .into_iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            );
            ret.push_str("\n");
        }

        insta::assert_display_snapshot!(ret, @r###"
        1, 2, 3, 4
        1, 2, 4, 3
        1, 4, 2, 3
        4, 1, 2, 3
        4, 1, 3, 2
        1, 4, 3, 2
        1, 3, 4, 2
        1, 3, 2, 4
        3, 1, 2, 4
        3, 1, 4, 2
        3, 4, 1, 2
        4, 3, 1, 2
        4, 3, 2, 1
        3, 4, 2, 1
        3, 2, 4, 1
        3, 2, 1, 4
        2, 3, 1, 4
        2, 3, 4, 1
        2, 4, 3, 1
        4, 2, 3, 1
        4, 2, 1, 3
        2, 4, 1, 3
        2, 1, 4, 3
        2, 1, 3, 4
        "###);
    }
}
