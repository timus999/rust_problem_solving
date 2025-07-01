pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut s = x.to_string();

        if s[0] == '-' {
            let s = s[1..s.len()];
        }

        s = s.rev();

        s.parse().unwrap()
    }
}

fn main() {
    assert_eq!(Solution::reverse(123), 321);
}
