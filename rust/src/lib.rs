
pub mod solution {
    pub fn merge_arrays(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (m, n) = (m as usize, n as usize);

        let mut mj =  match m {
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
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn test_add() {
        let mut nums1 = vec![1,2,3,0,0,0];
        let m = 3;
        let mut nums2 = vec![2,5,6];
        let n = 3;

        solution::merge_arrays(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1,2,2,3,5,6]);
    }

    #[test]
    fn test_add_empty_first() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;

        solution::merge_arrays(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_add_empty_second() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;

        solution::merge_arrays(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1]);
    }
}
