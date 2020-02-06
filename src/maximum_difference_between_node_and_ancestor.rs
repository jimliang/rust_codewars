#[derive(PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ val: {}, left: {}, right: {} ]",
            self.val,
            match self.left {
                Some(ref l) => format!("{:?}", l.borrow()),
                None => String::from("null"),
            },
            match self.right {
                Some(ref l) => format!("{:?}", l.borrow()),
                None => String::from("null"),
            }
        )
    }
}

use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn diff(node: Option<Rc<RefCell<TreeNode>>>, mut max: i32, mut min: i32) -> i32 {
            match node {
                Some(n) => {
                    let mut n_mut = n.borrow_mut();
                    let val = n_mut.val;
                    if max < val {
                        max = val;
                    }
                    if min > val {
                        min = val;
                    }
                    match (n_mut.left.take(), n_mut.right.take()) {
                        (None, None) => max - min,
                        (left_node, right_node) => {
                            let left = diff(left_node, max, min);
                            let right = diff(right_node, max, min);
                            left.max(right)
                        }
                    }
                }
                None => 0,
            }
        }
        if let Some(r) = root {
            let mut r_mut = r.borrow_mut();
            let left = diff(r_mut.left.take(), r_mut.val, r_mut.val);
            let right = diff(r_mut.right.take(), r_mut.val, r_mut.val);
            left.max(right)
        } else {
            0
        }
    }
}

fn build_tree(mut list: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn map_val(val: Option<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        val.and_then(|item| {
            if item == 0 {
                None
            } else {
                Some(Rc::new(RefCell::new(TreeNode::new(item))))
            }
        })
    }
    list.reverse();
    let root = Rc::new(RefCell::new(TreeNode::new(list.pop().unwrap())));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.clone());
    while let Some(node) = {
        if list.is_empty() {
            None
        } else {
            queue.pop_front()
        }
    } {
        let left = map_val(list.pop());
        let right = map_val(list.pop());
        if let Some(ref l) = left {
            queue.push_back(l.clone());
        }
        if let Some(ref r) = right {
            queue.push_back(r.clone());
        }
        let mut node_mut = node.borrow_mut();
        node_mut.left = left;
        node_mut.right = right;
    }
    Some(root)
}

#[test]
fn test_diff() {
    let tree = build_tree(vec![8, 3, 10, 1, 6, 0, 14, 0, 0, 4, 7, 13]);
    println!("{:?}", tree);
    assert_eq!(Solution::max_ancestor_diff(tree), 7);
}

// pass