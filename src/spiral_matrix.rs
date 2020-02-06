pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        #[derive(Debug, Clone, Copy)]
        enum Direction {
            UP,
            DOWN,
            LEFT,
            RIGHT,
        };
        struct SpiralIterator {
            offset_x: isize,
            offset_y: isize,
            width: isize,
            height: isize,
            matrix: Vec<Vec<i32>>,
            position: (isize, isize),
            direction: Direction,
        }

        impl SpiralIterator {
            fn new(matrix: Vec<Vec<i32>>) -> Self {
                assert!(matrix.len() > 0);
                assert!(matrix[0].len() > 0);
                SpiralIterator {
                    offset_x: 0,
                    offset_y: 0,
                    width: matrix[0].len() as isize,
                    height: matrix.len() as isize,
                    matrix,
                    position: (0, 0),
                    direction: Direction::RIGHT,
                }
            }
        }
        impl Iterator for SpiralIterator {
            type Item = i32;
            fn next(&mut self) -> Option<Self::Item> {
                let (x, y) = self.position;
                let index_y = y + self.offset_y;
                let index_x = x + self.offset_x;
                // println!(
                //     "postion ({:?}) => ({:?}) dir {:?} w,h ({:?})",
                //     self.position,
                //     (index_x, index_y),
                //     self.direction,
                //     (self.width, self.height)
                // );
                if x < 0 || y < 0 || x >= self.width || y >= self.height {
                    return None;
                }
                match self.direction {
                    Direction::RIGHT => {
                        if self.position.0 + 1 >= self.width {
                            self.offset_y += 1;
                            self.height -= 1;
                            self.direction = Direction::DOWN;
                        } else {
                            self.position.0 += 1;
                        }
                    }
                    Direction::LEFT => {
                        if self.position.0 <= 0 {
                            self.height -= 1;
                            self.direction = Direction::UP;
                            self.position.1 -= 1;
                        } else {
                            self.position.0 -= 1;
                        }
                    }
                    Direction::DOWN => {
                        if self.position.1 + 1 >= self.height {
                            self.width -= 1;
                            self.direction = Direction::LEFT;
                            self.position.0 -= 1;
                        } else {
                            self.position.1 += 1;
                        }
                    }
                    Direction::UP => {
                        if self.position.1 <= 0 {
                            self.offset_x += 1;
                            self.width -= 1;
                            self.direction = Direction::RIGHT;
                        } else {
                            self.position.1 -= 1;
                        }
                    }
                };

                // println!("{}", self.matrix[index_y as usize][index_x as usize]);
                Some(self.matrix[index_y as usize][index_x as usize])
            }
        }
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return vec![];
        }

        SpiralIterator::new(matrix).collect()
    }
}

#[test]
fn spiral_test() {
    assert_eq!(
        Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
    );
    assert_eq!(
        Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12]
        ]),
        vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
    );
    assert_eq!(
        Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16]
        ]),
        vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
    );
}
