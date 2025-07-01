pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut reverse = 0;

        let mut temp = x.abs();

        while temp > 0 {
            let rm = temp % 10;
            reverse = reverse * 10 + rm;
            temp = temp / 10;
        }

        if reverse == x {
            return true;
        }

        false
    }
}

fn main() {
    assert_eq!(Solution::is_palindrome(121), true);
}
