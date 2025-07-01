pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let (mut left, mut right) = (0, 0);
        let (mut long_left, mut long_right) = (0, 0);

        let mut chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        while left >= 0 && right < s.len() {
            if s.len() % 2 == 0 {
                right = i + 1;
                while left >= 0 && right < s.len() {
                    if chars[left] == chars[right] {
                        left -= 1;
                        right += 1;
                    }
                }
                if (long_right - long_left) < (right - left) {
                    long_left = left - 1;
                    long_right = right;
                }
            } else {
                right = i + 1;
                left = i - 1;
                while left >= 0 && right < s.len() {
                    if chars[left] == chars[right] {
                        left -= 1;
                        right += 1;
                    }
                }
                if (long_right - long_left) < (right - left) {
                    long_left = left + 1;
                    long_right = right;
                }
            }
        }
        chars[left..right].to_string()
    }
}

fn main() {
    assert_eq!(
        Solution::longest_palindrome(String::from("babad")),
        String::from("bab")
    );

    assert_eq!(
        Solution::longest_palindrome(String::from("cbbd")),
        String::from("bb")
    );
}
