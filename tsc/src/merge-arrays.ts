/**
 Do not return anything, modify nums1 in-place instead.
 */
export function mergeArrays(
  nums1: number[],
  m: number,
  nums2: number[],
  n: number
): void {
  let mi = m - 1;
  let nj = n - 1;

  for (let i = 0; i < m + n; i++) {
    if (nj < 0) {
      nums1[m + n - 1 - i] = nums1[mi];
      mi--;
      continue;
    }

    if (mi < 0) {
      nums1[m + n - 1 - i] = nums2[nj];
      nj--;
      continue;
    }

    if (nums1[mi] > nums2[nj]) {
      nums1[m + n - 1 - i] = nums1[mi];
      mi--;
    } else {
      nums1[m + n - 1 - i] = nums2[nj];
      nj--;
    }
  }
}
