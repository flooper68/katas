package solution

import (
	"reflect"
	"testing"
)

type crystalBallsTest struct {
	title    string
	breaks   []bool
	expected int
}

var crystalBallsTests = []crystalBallsTest{
	{"finds the breaking point", []bool{false, false, false, false, false, false, false, true}, 7},
	{"finds the breaking point", []bool{false, false, false, false, false, false, true, true, true, true}, 6},
	{"finds the breaking point", []bool{false, false, false, false, false, false, false, false, false, true}, 9},
	{"finds the breaking point if there is just single value", []bool{true}, 0},
	{"returns -1 if there is not breaking point", []bool{false, false}, -1},
	{"returns -1 for empty array", []bool{}, -1},
}

func TestCrystalBalls(t *testing.T) {
	for _, test := range crystalBallsTests {
		result := twoCrystalBalls(test.breaks)

		if !reflect.DeepEqual(result, test.expected) {
			t.Error("twoCrystalBalls - "+test.title, ",expected:", test.expected, ",got:", result)
		} else {
			t.Log("twoCrystalBalls - " + test.title)
		}
	}
}
