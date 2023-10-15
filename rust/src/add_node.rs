#[derive(Debug, PartialEq)]
pub struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

/**
 * This implementation is different from other languages because we can not create the value here and return
 * it as a reference because the value will be dropped at the end of the function. The create method and the add_node
 * must be separate and be called in higher scope. Also this would probably be nicer as a struct method.
 */
pub fn add_node(root: &mut Box<Node>, value: i32) {
    if value < root.val {
        match &mut root.left {
            None => {
                root.left = Some(Box::new(Node {
                    val: value,
                    left: None,
                    right: None,
                }));
            }
            Some(node) => {
                add_node(node, value);
            }
        };
    } else {
        match &mut root.right {
            None => {
                root.right = Some(Box::new(Node {
                    val: value,
                    left: None,
                    right: None,
                }));
            }
            Some(node) => add_node(node, value),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::add_node::*;

    struct Test {
        title: &'static str,
        root: Box<Node>,
        value: i32,
        expected: Box<Node>,
    }

    #[test]
    fn test_add_node() {
        let mut test_cases = vec![
            Test {
                title: "adds node to left",
                root: Box::new(Node {
                    val: 2,
                    left: None,
                    right: None,
                }),
                value: 1,
                expected: Box::new(Node {
                    val: 2,
                    left: Some(Box::new(Node {
                        val: 1,
                        left: None,
                        right: None,
                    })),
                    right: None,
                }),
            },
            Test {
                title: "adds node to right",
                root: Box::new(Node {
                    val: 2,
                    left: None,
                    right: None,
                }),
                value: 3,
                expected: Box::new(Node {
                    val: 2,
                    left: None,
                    right: Some(Box::new(Node {
                        val: 3,
                        left: None,
                        right: None,
                    })),
                }),
            },
            Test {
                title: "adds node to right if they are equal",
                root: Box::new(Node {
                    val: 2,
                    left: None,
                    right: None,
                }),
                value: 2,
                expected: Box::new(Node {
                    val: 2,
                    left: None,
                    right: Some(Box::new(Node {
                        val: 2,
                        left: None,
                        right: None,
                    })),
                }),
            },
            Test {
                title: "adds node deep in a tree",
                root: Box::new(Node {
                    val: 10,
                    left: Some(Box::new(Node {
                        val: 2,
                        left: None,
                        right: Some(Box::new(Node {
                            val: 5,
                            left: Some(Box::new(Node {
                                val: 4,
                                left: None,
                                right: None,
                            })),
                            right: None,
                        })),
                    })),
                    right: None,
                }),
                value: 3,
                expected: Box::new(Node {
                    val: 10,
                    left: Some(Box::new(Node {
                        val: 2,
                        left: None,
                        right: Some(Box::new(Node {
                            val: 5,
                            left: Some(Box::new(Node {
                                val: 4,
                                left: Some(Box::new(Node {
                                    val: 3,
                                    left: None,
                                    right: None,
                                })),
                                right: None,
                            })),
                            right: None,
                        })),
                    })),
                    right: None,
                }),
            },
        ];

        test_cases.iter_mut().for_each(|test| {
            add_node(&mut test.root, test.value);

            assert_eq!(test.root, test.expected, "add_node - {}", test.title);
        });
    }
}
