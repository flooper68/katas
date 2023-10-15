package solution

import (
	"reflect"
	"testing"
)

type addNodeTest struct {
	title    string
	root     *Node
	value    int
	expected *Node
}

var addNodeTests = []addNodeTest{
	{"creates new root if root is nill", nil, 4, &Node{4, nil, nil}},
	{"adds value to left", &Node{10, nil, nil}, 4, &Node{10, &Node{4, nil, nil}, nil}},
	{"adds value to right", &Node{10, nil, nil}, 11, &Node{10, nil, &Node{11, nil, nil}}},
	{"adds value to right if it is equal", &Node{10, nil, nil}, 10, &Node{10, nil, &Node{10, nil, nil}}},
	{"adds value to complicated position in the tree", &Node{10, &Node{4, &Node{2, nil, nil}, nil}, &Node{11, nil, nil}}, 3, &Node{10, &Node{4, &Node{2, nil, &Node{3, nil, nil}}, nil}, &Node{11, nil, nil}}},
}

func TestAddNode(t *testing.T) {
	for _, test := range addNodeTests {
		result := addNode(test.root, test.value)

		if !reflect.DeepEqual(result, test.expected) {
			t.Error("addNode - "+test.title+",expected:", test.expected, ",got:", result)
		} else {
			t.Log("addNode - " + test.title)
		}
	}
}
