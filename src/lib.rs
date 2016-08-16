pub fn quicksort<T: PartialEq + PartialOrd>(vec: &mut [T]) -> &mut [T] {
    if vec.len() <= 1 {
        return vec;
    }
    let pivot: usize = 0;
    let mut first_opened = Vec::new();
    let mut last_closed: usize = 0;

    for i in 1..vec.len() {
        first_opened.push(i);
        if vec[i] < vec[pivot] {
            let to = first_opened.remove(0);
            vec.swap(i, to);
            last_closed = to;
        }
    }

    vec.swap(pivot, last_closed);
    quicksort(&mut vec[0..last_closed]);
    quicksort(&mut vec[last_closed+1..]);
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_of_u64() {
        let sorted: Vec<u64> = vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9];
        let mut unsorted: Vec<u64> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        quicksort(&mut unsorted);

        assert_eq!(unsorted, sorted.as_slice());
    }

    #[test]
    fn test_vec_of_u8() {
        let sorted = "        Tabcdeeefghhijklmnoooopqrrstuuvwxyz".to_string().into_bytes();
        let mut unsorted = "The quick brown fox jumps over the lazy dog".to_string().into_bytes();
        quicksort(&mut unsorted);

        assert_eq!(sorted, unsorted);
    }
}
