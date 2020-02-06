// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}
pub struct Solution;
impl Solution {
  pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
      return None;
    }
    let mut values = vec![];
    let mut list = vec![];
    let mut cur = &head;
    while let Some(c) = cur {
      values.push(c.val);
      if values.len() >= k as usize {
        values.reverse();
        list.extend(values.drain(..));
      }
      cur = &c.next;
    }
    if !values.is_empty() {
      list.extend(values);
    }
    build_node(list)
  }
  pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn take_childs(mut parent: &mut Box<ListNode>) -> Option<(Box<ListNode>, Box<ListNode>)> {
      if let Some(ref c) = parent.next {
        if c.next.is_some() {
          let mut child = parent.next.take().unwrap();
          let son = child.next.take().unwrap();
          return Some((child, son));
        }
      }
      None
    }
    fn swap(parent: &mut Box<ListNode>) {
      if let Some((mut child, mut son)) = take_childs(parent) {
        child.next = son.next.take();
        swap(&mut child);
        son.next = Some(child);
        parent.next = Some(son);
      }
    }
    let mut root = Box::new(ListNode { val: 0, next: head });
    swap(&mut root);
    root.next.take()
  }
}

fn build_node(mut list: Vec<i32>) -> Option<Box<ListNode>> {
  if list.is_empty() {
    None
  } else {
    let mut cur = ListNode::new(list.pop().unwrap());
    while let Some(val) = list.pop() {
      cur = ListNode {
        val,
        next: Some(Box::new(cur)),
      };
    }
    Some(Box::new(cur))
  }
}

fn build_list(mut node: Option<Box<ListNode>>) -> Vec<i32> {
  let mut list = vec![];
  let mut cur = &node;
  while let Some(c) = cur {
    list.push(c.val);
    cur = &c.next;
  }
  list
}

#[test]
fn build_test() {
  let list = build_node(vec![1, 2, 3, 4, 5]);
  println!(
    "{:?}",
    build_list(Solution::reverse_k_group(list.clone(), 2))
  );
  println!("{:?}", build_list(Solution::reverse_k_group(list, 3)));
}

#[test]
fn build_test2() {
  let list = build_node(vec![1, 2, 3, 4, 5]);
  println!("{:?}", build_list(Solution::swap_pairs(list)))
}
