interface Node {
  value: number;
  left: Node | null;
  right: Node | null;
}

export function addNode(root: Node | null, value: number) {
  if (root === null) {
    return { value, left: null, right: null };
  }

  if (value < root.value) {
    if (root.left === null) {
      root.left = { value, left: null, right: null };
    } else {
      addNode(root.left, value);
    }
  } else {
    if (root.right === null) {
      root.right = { value, left: null, right: null };
    } else {
      addNode(root.right, value);
    }
  }

  return root;
}
