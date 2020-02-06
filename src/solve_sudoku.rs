// https://leetcode-cn.com/problems/sudoku-solver/
pub struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        use std::collections::HashSet;
        fn get_empty_cell(board: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
            let mut cells = vec![];
            for (r, row) in board.iter().enumerate() {
                for (c, col) in row.iter().enumerate() {
                    if *col == '.' {
                        cells.push((r, c))
                    }
                }
            }
            cells
        }

        fn guess_value(board: &Vec<Vec<char>>, cell: &(usize, usize)) -> HashSet<char> {
            let mut chars: HashSet<_> = "123456789".chars().collect();
            // check row
            for (c, col) in board[cell.0].iter().enumerate() {
                if c == cell.1 || *col == '.' {
                    continue;
                }
                chars.remove(col);
            }

            // check col
            for row in board.iter() {
                if row[cell.1] == '.' {
                    continue;
                }
                chars.remove(&row[cell.1]);
            }

            // check box
            let begin_r = (cell.0 / 3) * 3;
            let begin_c = (cell.1 / 3) * 3;
            for r in begin_r..begin_r + 3 {
                for c in begin_c..begin_c + 3 {
                    if board[r][c] == '.' {
                        continue;
                    }
                    chars.remove(&board[r][c]);
                }
            }

            chars
        }

        fn get_close_empty(board: &Vec<Vec<char>>) -> Option<((usize, usize), HashSet<char>)> {
            let empty_cells = get_empty_cell(&board);
            let mut empty: Vec<_> = empty_cells
                .into_iter()
                .map(|cell| (cell, guess_value(&board, &cell)))
                .collect();
            empty.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
            empty.first().cloned()
        }

        fn dfs(board: &mut Vec<Vec<char>>) -> bool {
            if let Some((cell, selections)) = get_close_empty(&board) {
                // let selections = guess_value(&board, &cell);
                if selections.is_empty() {
                    // println!("backtrack {}", "empty selections");
                    return false;
                }
                // let len = selections.len();
                for selection in selections {
                    // println!("try {:?} -> {}/ ({})", cell, selection, len);
                    board[cell.0][cell.1] = selection;
                    if dfs(board) {
                        return true;
                    }
                }
                board[cell.0][cell.1] = '.';
                // println!("backtrack {}", "selections not match");
                false
            } else {
                true
            }
        }
        let _ = dfs(board);
    }
}

fn print_board(board: &Vec<Vec<char>>) {
    use std::iter::FromIterator;
    for row in board {
        println!("{}", String::from_iter(row));
    }
}

fn build_board(s: impl Into<String>) -> Vec<Vec<char>> {
    let chars: Vec<char> = s.into().chars().collect();
    chars.chunks(9).map(|row| Vec::from(row)).collect()
}

fn build_board2(s: [[&str; 9]; 9]) -> Vec<Vec<char>> {
    s.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cell| cell.chars().next().unwrap())
                .collect()
        })
        .collect()
}

#[test]
fn name() {
    let board_str =
        "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
    let mut board = build_board(board_str);
    print_board(&board);
    Solution::solve_sudoku(&mut board);
    println!("{}", "======");
    print_board(&board);
}

#[test]
fn name2() {
    let board = [
        [".", ".", ".", "2", ".", ".", ".", "6", "3"],
        ["3", ".", ".", ".", ".", "5", "4", ".", "1"],
        [".", ".", "1", ".", ".", "3", "9", "8", "."],
        [".", ".", ".", ".", ".", ".", ".", "9", "."],
        [".", ".", ".", "5", "3", "8", ".", ".", "."],
        [".", "3", ".", ".", ".", ".", ".", ".", "."],
        [".", "2", "6", "3", ".", ".", "5", ".", "."],
        ["5", ".", "3", "7", ".", ".", ".", ".", "8"],
        ["4", "7", ".", ".", ".", "1", ".", ".", "."],
    ];
    let mut board = build_board2(board);
    print_board(&board);
    Solution::solve_sudoku(&mut board);
    println!("{}", "======");
    print_board(&board);
}
