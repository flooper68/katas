package solution

import (
	"reflect"
	"testing"
)

type compressTest struct {
  title string
  input string
  indexes []int
  chars []rune
}

var compressTests = []compressTest{
  {"compress should work with abcab", "abcab", []int{1,2,3,1,2}, []rune{'a','b','c'}},
}

func TestCompress(t *testing.T) {
  for _, test := range compressTests {
    indexes, chars := compress(test.input)


    if !reflect.DeepEqual(indexes ,test.indexes) || !reflect.DeepEqual(chars, test.chars) {
      t.Error("compress - " + test.title + ",expected:", test.indexes, test.chars, ",got:", indexes, chars)
    } else {
      t.Log("compress - " + test.title)
    }
  }
}
