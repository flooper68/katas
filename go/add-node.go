package solution

type Node struct {
	value int
	left  *Node
	right *Node
}

func addNode(root *Node, value int) *Node {
	if root == nil {
		return &Node{value, nil, nil}
	}

	if value < root.value {
		if root.left == nil {
			root.left = &Node{value, nil, nil}
		} else {
			addNode(root.left, value)
		}
	} else {
		if root.right == nil {
			root.right = &Node{value, nil, nil}
		} else {
			addNode(root.right, value)
		}
	}

	return root
}
