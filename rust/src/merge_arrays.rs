pub fn merge_arrays(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (m, n) = (m as usize, n as usize);

    let mut mj = match m {
        0 => 0,
        _ => m - 1,
    };
    let mut nj = match n {
        0 => 0,
        _ => n - 1,
    };
    let combined = m + n;

    let mut m_stopped = match m {
        0 => true,
        _ => false,
    };
    let mut n_stopped = match n {
        0 => true,
        _ => false,
    };

    for i in 0..m + n {
        if n_stopped {
            nums1[combined - 1 - i] = nums1[mj];

            if mj != 0 {
                mj -= 1;
            }
            continue;
        }

        if m_stopped {
            nums1[combined - 1 - i] = nums2[nj];

            if nj != 0 {
                nj -= 1;
            }
            continue;
        }

        if nums1[mj] > nums2[nj] {
            nums1[combined - 1 - i] = nums1[mj];

            if mj != 0 {
                mj -= 1;
            } else {
                m_stopped = true;
            }
        } else {
            nums1[combined - 1 - i] = nums2[nj];

            if nj != 0 {
                nj -= 1;
            } else {
                n_stopped = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::merge_arrays::*;

    struct Test {
        title: &'static str,
        nums1: Vec<i32>,
        m: i32,
        nums2: Vec<i32>,
        n: i32,
    }

    #[test]
    fn test_add() {
        let mut test_cases = vec![
            Test {
                title: "merges two arrays",
                nums1: vec![1, 2, 3, 0, 0, 0],
                m: 3,
                nums2: vec![2, 5, 6],
                n: 3,
            },
            Test {
                title: "merges two arrays when first array is empty",
                nums1: vec![0],
                m: 0,
                nums2: vec![1],
                n: 1,
            },
            Test {
                title: "merges two arrays when second array is empty",
                nums1: vec![1],
                m: 1,
                nums2: vec![],
                n: 0,
            },
        ];

        test_cases.iter_mut().for_each(|test| {
            merge_arrays(&mut test.nums1, test.m, &mut test.nums2, test.n);

            assert_eq!(
                test.nums1,
                vec![1, 2, 2, 3, 5, 6],
                "compress - {}",
                test.title
            );
        });
    }
}
