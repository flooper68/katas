package solution

import (
	"fmt"
	"reflect"
	"testing"
)

func TestBasicCase(t *testing.T) {
  var nums1 = []int{1,2,3,0,0,0}
  var m = 3
  var nums2 = []int{2,5,6}
  var n = 3


  mergeArrays(nums1, m, nums2, n)

  if !reflect.DeepEqual(nums1, []int{1,2,2,3,5,6}) {
    fmt.Println("Expected [1,2,2,3,5,6] but got ", nums1)
    t.Error("FAIL")
  }
}

func TestFirstEmpty(t *testing.T) {
  var nums1 = []int{0}
  var m = 0
  var nums2 = []int{1}
  var n = 1


  mergeArrays(nums1, m, nums2, n)

  if !reflect.DeepEqual(nums1, []int{1}) {
    fmt.Println("Expected [1] but got ", nums1)
    t.Error("FAIL")
  }
}

func TestSecondEmpty(t *testing.T) {
  var nums1 = []int{1}
  var m = 1
  var nums2 = []int{}
  var n = 0


  mergeArrays(nums1, m, nums2, n)

  if !reflect.DeepEqual(nums1, []int{1}) {
    fmt.Println("Expected [1] but got ", nums1)
    t.Error("FAIL")
  }
}