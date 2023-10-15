pub fn binary_search(arr: &Vec<i32>, value: i32) -> i32 {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = (low + high) / 2;

        match arr[mid] {
            v if v == value => return mid as i32,
            v if v < value => low = mid + 1,
            _ => high = mid,
        }
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use crate::binary_search::*;

    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search(&vec![1, 2, 3], 1), 0);
        assert_eq!(binary_search(&vec![1, 2, 3], 2), 1);
        assert_eq!(binary_search(&vec![1, 2, 3], 3), 2);
        assert_eq!(binary_search(&vec![1], 1), 0);
        assert_eq!(binary_search(&vec![], 3), -1);
        assert_eq!(binary_search(&vec![1, 2, 3], 4), -1);
    }
}
