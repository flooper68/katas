package solution

import (
	"reflect"
	"testing"
)

type mergeArraysTest struct {
  title string
  nums1 []int
  m int
  nums2 []int
  n int
  expected []int
}

var mergeArraysTests = []mergeArraysTest{
  {"merges general arrays", []int{1,2,3,0,0,0}, 3, []int{2,5,6}, 3, []int{1,2,2,3,5,6}},
  {"merges when first array is empty", []int{0}, 0, []int{1}, 1, []int{1}},
  {"merges when second array is empty", []int{1}, 1, []int{}, 0, []int{1}},

}

func TestMergeArray(t *testing.T) {
  for _, test := range mergeArraysTests {
    mergeArrays(test.nums1, test.m, test.nums2, test.n)

    if !reflect.DeepEqual(test.nums1, test.expected) {
      t.Error("isAscSorted - " + test.title, ",expected:", test.expected, ",got:", test.nums1)
    } else {
      t.Log("mergeArrays - " + test.title)
    }
  }
}
