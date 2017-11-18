
use std::collections::BinaryHeap;

fn descending_order(x: u64) -> u64 {
    let mut list = BinaryHeap::new();
    let mut xx = x;
    let num = 10;
    while xx > 0 {
        list.push(xx % num);
        xx = xx / num
    }
    let mut result = 0;
    while let Some(i) = list.pop() {
        result = result * num + i;
    }
    result
}

#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}