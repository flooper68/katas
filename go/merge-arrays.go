package solution


func mergeArrays(nums1 []int, m int, nums2 []int, n int)  {

  var combined = m + n
  var mi = m - 1
  var ni = n - 1

  for i := combined - 1; i >= 0; i-- {
    if mi < 0 {
      nums1[i] = nums2[ni]
      ni--
      continue;
    }

    if(ni < 0) {
      nums1[i] = nums1[mi]
      mi--
      continue;
    }

    if nums1[mi] < nums2[ni] {
      nums1[i] = nums2[ni]
      ni--
    } else {
      nums1[i] = nums1[mi]
      mi--
    }
  }
}
