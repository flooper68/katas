pub fn is_asc_sorted(array: &Vec<i32>) -> bool {
    if array.len() == 0 {
        return true;
    }

    for i in 0..array.len() - 1 {
        if array[i] > array[i + 1] {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use crate::is_asc_sorted::*;

    #[test]
    fn test_is_asc_sorted() {
        assert_eq!(is_asc_sorted(&vec![1, 2, 3]), true);
        assert_eq!(is_asc_sorted(&vec![1, 2, 2, 3]), true);
        assert_eq!(is_asc_sorted(&vec![1]), true);
        assert_eq!(is_asc_sorted(&vec![]), true);
        assert_eq!(is_asc_sorted(&vec![3, 2, 1]), false);
        assert_eq!(is_asc_sorted(&vec![1, 3, 2]), false);
    }
}
