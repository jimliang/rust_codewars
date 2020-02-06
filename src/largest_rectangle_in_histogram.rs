struct Solution;

impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        heights.push(0);
        let mut stack: Vec<i32> = vec![];
        let mut max_area = 0;
        for i in 0..heights.len() {
            while stack
                .last()
                .map(|last| heights[*last as usize] > heights[i])
                .unwrap_or(false)
            {
                let last = stack.pop().unwrap();
                let cur_area = heights[last as usize] * match stack.last() {
                    Some(last2) => i as i32 - last2 - 1,
                    None => i as i32,
                };
                max_area = max_area.max(cur_area);
                // println!("i {}, cur_area {}", i, cur_area);
            }
            stack.push(i as i32);
        }
        max_area
    }
}

#[test]
fn largest_rectangle_area() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
}