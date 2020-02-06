pub struct Solution;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut left = 0;
        let mut right = 0;
        let chars: Vec<_> = s.chars().collect();
        for ch in &chars {
            match ch {
                '(' => {
                    left += 1;
                }
                ')' => {
                    if left == 0 {
                        right += 1;
                    } else {
                        left -= 1;
                    }
                }
                _ => {}
            }
        }
        let mut expression = Vec::with_capacity(chars.len());
        let mut result = std::collections::HashSet::new();
        recurse(&chars, 0, 0, 0, left, right, &mut expression, &mut result);
        result.into_iter().collect()
    }
}

fn recurse(
    chars: &Vec<char>,
    index: usize,
    left: usize,
    right: usize,
    left_rem: usize,
    right_rem: usize,
    expression: &mut Vec<char>,
    result: &mut std::collections::HashSet<String>,
) {
    use std::iter::FromIterator;
    if index == chars.len() {
        if left_rem == 0 && right_rem == 0 {
            result.insert(String::from_iter(expression.iter().clone()));
        }
        return;
    }
    let ch = chars[index];
    if (ch == '(' && left_rem > 0) || (ch == ')' && right_rem > 0) {
        let left_rem = if ch == '(' { left_rem - 1 } else { left_rem };
        let right_rem = if ch == ')' { right_rem - 1 } else { right_rem };
        recurse(
            chars,
            index + 1,
            left,
            right,
            left_rem,
            right_rem,
            expression,
            result,
        );
    }
    expression.push(ch);
    if ch != '(' && ch != ')' {
        recurse(
            chars,
            index + 1,
            left,
            right,
            left_rem,
            right_rem,
            expression,
            result,
        );
    } else if ch == '(' {
        recurse(
            chars,
            index + 1,
            left + 1,
            right,
            left_rem,
            right_rem,
            expression,
            result,
        );
    } else if right < left {
        recurse(
            chars,
            index + 1,
            left,
            right + 1,
            left_rem,
            right_rem,
            expression,
            result,
        );
    }
    let _ = expression.pop();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_eq(s: &str, list: Vec<&str>) {
        let result = Solution::remove_invalid_parentheses(s.to_string());
        assert_eq!(result.len(), list.len());
        for item in list {
            assert!(result.contains(&item.to_string()));
        }
    }
    #[test]
    fn remove_invalid_parentheses() {
        // assert_eq!(
        //     Solution::remove_invalid_parentheses("()())()".to_string()),
        //     vec!["()()()".to_string(), "(())()".to_string()]
        // );
        assert_eq("()())()", vec!["()()()", "(())()"]);
        assert_eq("(a)())()", vec!["(a)()()", "(a())()"]);
        assert_eq(")(", vec![""]);
    }
}

// https://leetcode-cn.com/problems/remove-invalid-parentheses
// pass
