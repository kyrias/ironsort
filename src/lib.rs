//! # quicksort
//!
//! Quicksort is an efficient in-place sorting algorithm created by Tony Hoare.
//!

use std::cmp::Ordering;


/// In-place sorting of a slice of T.
///
/// ```rust
/// # use ironsort::quicksort;
/// let presorted: Vec<u64> = vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9];
/// let mut vector: Vec<u64> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
/// quicksort(&mut vector);
///
/// assert_eq!(vector, presorted.as_slice());
/// ```
#[inline]
pub fn quicksort<T: Ord>(vec: &mut [T]) {
    quicksort_by(vec, &|x, y| x.cmp(y))
}


/// In-place sorting of a slice of T using a custom comparison function.
///
/// ```rust
/// # use ironsort::quicksort_by;
/// # use std::cmp::Ordering;
///
/// let presorted: Vec<u64> = vec![9, 6, 5, 5, 4, 3, 3, 2, 1, 1];
/// let mut vector: Vec<u64> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
///
/// quicksort_by(&mut vector, &|x, y| x.cmp(y).reverse());
///
/// assert_eq!(vector, presorted.as_slice());
/// ```
pub fn quicksort_by<T: PartialOrd, F>(vec: &mut [T], cmp: &F)
    where F: Fn(&T, &T) -> Ordering {

    let len: usize = vec.len();
    if len <= 1 {
        return;
    }

    let pivot: usize = 0;
    vec.swap(pivot, len / 2);

    let mut left: usize = 0;
    let mut right: usize = vec.len() - 1;

    while left < right {
        while left < len && cmp(&vec[left], &vec[pivot]) != Ordering::Greater {
            left += 1
        }
        while cmp(&vec[right], &vec[pivot]) == Ordering::Greater {
            right -= 1
        }

        if left < right {
            vec.swap(left, right);
        }
    }

    vec.swap(pivot, right);
    quicksort_by(&mut vec[0..right], cmp);
    quicksort_by(&mut vec[right+1..], cmp);
}


#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod tests {
    use rand::{self, Rng};
    use super::*;

    #[test]
    fn test_vec_of_u64() {
        let presorted: Vec<u64> = vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9];
        let mut vector: Vec<u64> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        quicksort(&mut vector);

        assert_eq!(vector, presorted.as_slice());
    }

    #[test]
    fn test_vec_of_u8() {
        let presorted = "        Tabcdeeefghhijklmnoooopqrrstuuvwxyz".to_string().into_bytes();
        let mut vector = "The quick brown fox jumps over the lazy dog".to_string().into_bytes();
        quicksort(&mut vector);

        assert_eq!(presorted, vector);
    }

    #[test]
    fn test_array_of_u8() {
        let presorted: [u8; 10] = [1, 2, 3, 4, 5, 5, 5, 7, 9, 10];
        let mut vector: [u8; 10] = [7, 5, 3, 2, 5, 1, 4, 5, 9, 10];
        quicksort(&mut vector);

        assert_eq!(presorted, vector);
    }

    #[test]
    fn test_random() {
        let mut rng = rand::thread_rng();

        for _ in 0u64 .. 10_000u64 {
            let len: usize = rng.gen();
            let mut vector = rng.gen_iter()
                                .take((len % 64) + 1)
                                .collect::<Vec<usize>>();
            quicksort(&mut vector);

            for i in 0 .. vector.len() - 1 {
                assert!(vector[i] <= vector[i + 1])
            }
        }
    }
}
