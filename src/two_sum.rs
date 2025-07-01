// two sum problem solution in rust
//

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let out: Vec<i32> = vec![];
        let result = HashMap::new();

        for i in 0..nums.len() {
            let res = target - nums[i];

            if result.contains(res) {
                out.push(nums[i]);
                out.push(res);
                return out;
            } else {
                result.insert(nums[i])
            }
        }

        out
    }
}

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}
