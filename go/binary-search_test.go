package solution

import (
	"reflect"
	"testing"
)

type binarySearchTest struct {
	title    string
	arr      []int
	value    int
	expected int
}

var binarySearchTests = []binarySearchTest{
	// {"binarySearch should work", []int{1, 2, 3, 1, 2}, 3, 2},
	// {"binarySearch return -1 for empty array", []int{}, 1, -1},
	{"binarySearch return -1 if the value is not found", []int{1, 2, 3}, 5, -1},
	// {"binarySearch return found index for array with single element", []int{1}, 1, 0},
}

func TestBinarySearch(t *testing.T) {
	for _, test := range binarySearchTests {
		result := binarySearch(test.arr, test.value)

		if !reflect.DeepEqual(result, test.expected) {
			t.Error("binarySearch - "+test.title+",expected:", test.expected, ",got:", result)
		} else {
			t.Log("binarySearch - " + test.title)
		}
	}
}
