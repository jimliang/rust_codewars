// https://leetcode-cn.com/problems/3sum

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        use std::collections::HashMap;

        fn range<F: FnMut(usize, usize)>(len: usize, mut callback: F) {
            for i in 0..len {
                for j in i..len {
                    callback(i, j);
                }
            }
        }
        let mut map = HashMap::new();
        for num in nums {
            let counter = map.entry(num).or_insert(0);
            *counter += 1;
        }
        let (a, b): (Vec<i32>, Vec<i32>) = map.keys().partition(|value| **value < 0);
        let mut result = vec![];
        let mut full = |list: &Vec<i32>| {
            range(list.len(), |i, j| {
                // 两个相同的情况
                if i == j && *map.get(&list[i]).unwrap() <= 1 {
                    return;
                }
                // 三个相同的情况
                if list[i] == 0 && list[j] == 0 && *map.get(&list[i]).unwrap() <= 2 {
                    return;
                }
                // 三个不同的情况
                let value = list[i] + list[j];
                if map.contains_key(&-value) {
                    result.push(vec![list[i], list[j], -value])
                }
            });
        };
        full(&a);
        full(&b);
        result
    }
}

#[test]
fn num3() {
    let list = vec![-1, 0, 1, 2, -1, -4];
    println!("{:?}", Solution::three_sum(list));
}

#[test]
fn num4() {
    let list = vec![-1, 0, 1, 0];
    println!("{:?}", Solution::three_sum(list));
}
#[test]
fn num5() {
    let list = vec![0, 0, 0];
    println!("{:?}", Solution::three_sum(list));
}
