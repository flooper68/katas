package solution

import (
	"testing"
)

type isAscSortedTest struct {
  title string
  array []int
  expected bool
}

var addTests = []isAscSortedTest{
  {"returns true for sorted array", []int{1,2,3}, true},
  {"returns true for sorted array with subsequent equal values", []int{1,2,2,3}, true},
  {"returns true for empty array", []int{}, true},
  {"returns true for array with single element", []int{1}, true},
  {"returns false for unsorted array", []int{1,3,2}, false},
  {"returns false for desc sorted array", []int{3,2,1}, false},
}

func TestIsAscSorted(t *testing.T) {
  for _, test := range addTests {
    result := isAscSorted(test.array)


    if result != test.expected {
      t.Error("isAscSorted - " + test.title, ",expected:", test.expected, ",got:", result)
    } else {
      t.Log("isAscSorted - " + test.title)
    }
  }
}
