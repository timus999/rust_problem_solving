use std::collections::HashSet;
pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;

        let mut checked = HashSet::new();
        let mut max = 0;
        for c in s.chars() {
            if Some(checked.contains(&c)) {
                longest = max;
                max = 0;
                checked.clear();
            }

            max += 1;
        }

        max as i32
    }
}

fn main() {
    assert_eq!(Solution::length_of_longest_substring("abcabcbb"), 3)
}
