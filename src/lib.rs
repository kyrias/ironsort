//! # quicksort
//!
//! Quicksort is an efficient in-line sorting algorithm created by Tony Hoare.
//!

use std::cmp::Ordering;

/// In-line sorting of a slice of T.
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
    quicksort_by(vec, &Ord::cmp)
}

pub fn quicksort_by<T: PartialOrd, F>(vec: &mut [T], cmp: &F)
    where F: Fn(&T, &T) -> Ordering {
    if vec.len() <= 1 {
        return;
    }
    let pivot: usize = 0;
    let mut first_opened = Vec::new();
    let mut last_closed: usize = 0;

    for i in 1..vec.len() {
        first_opened.push(i);
        match cmp(&vec[i], &vec[pivot]) {
            Ordering::Less => {
                let to = first_opened.remove(0);
                vec.swap(i, to);
                last_closed = to;
            }
            _ => { }
        }
    }

    vec.swap(pivot, last_closed);
    quicksort_by(&mut vec[0..last_closed], cmp);
    quicksort_by(&mut vec[last_closed+1..], cmp);
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
