pub struct Solution;

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        type Map = std::collections::HashMap<i32, i32>;
        fn new_map() -> Map {
            Map::with_capacity(2)
        }
        fn upper(map: &Map, index: &i32, value: &i32) -> bool {
            match map.get(index) {
                Some(v) => v > value,
                None => true,
            }
        }
        // [1,2,3,4], 1 => 2
        fn sect(arr: &Vec<i32>, value: &i32) -> Option<i32> {
            for v in arr.iter() {
                if v > value {
                    return Some(*v);
                }
            }
            None
        }
        arr2.sort();
        let mut dp: Map = new_map();
        dp.insert(0, arr1[0]);
        dp.insert(1, arr2[0]);

        for i in 1..arr1.len() {
            let mut new_dp = new_map();
            for (k, v) in dp.iter() {
                if arr1[i] > *v && upper(&new_dp, k, &arr1[i]) {
                    new_dp.insert(*k, arr1[i]);
                }
                let j = sect(&arr2, v);
                if let Some(j) = j {
                    if upper(&new_dp, &(k + 1), &j) {
                        new_dp.insert(k + 1, j);
                    }
                }
            }
            dp = new_dp;
        }
        *dp.keys().min().unwrap_or(&-1)
    }
}

#[test]
fn test_make_array_increasing() {
    assert_eq!(
        Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 3, 2, 4]),
        1
    );
    assert_eq!(
        Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![4, 3, 1]),
        2
    );
    assert_eq!(
        Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 6, 3, 3]),
        -1
    );
}
