pub struct Solution {}

impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> Vec<i32> {
        let n: usize = nums.len();

        nums.sort();

        let mut res: Vec<i32> = Vec::new();

        let max = n / 3;
        let mut count = 1;

        for i in 0..n - 1 as usize {
            if nums[i] == nums[i + 1] {
                count += 1;
            } else {
                if count > max {
                    res.push(nums[i]);
                }
                count = 1;
            }
        }
        if count > max {
            res.push(nums[n - 1]);
        }

        res
    }
}
fn main() {
    println!("{:?}", Solution::majority_element(vec![1]));
}
