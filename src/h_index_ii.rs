// https://leetcode-cn.com/problems/h-index-ii

pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let len = citations.len() as i32;
        if len == 0 {
            return 0;
        }
        let mut l = 0;
        let mut r = len - 1;
        while l < r {
            let mid = (l + r) / 2;
            if citations[mid as usize] >= (len - mid) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        if citations[r as usize] < (len - r) {
            0
        } else {
            len - r
        }
    }
}

#[test]
fn test_h_index() {
    assert_eq!(Solution::h_index(vec![0, 1, 3, 5, 6]), 3);
}
