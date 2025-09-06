use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut res = vec![intervals[0].clone()];

        for interval in intervals.into_iter().skip(1) {
            let last = res.last_mut().unwrap();

            if interval[0] <= last[1] {
                last[1] = std::cmp::max(interval[1], last[1]);
            } else {
                res.push(interval);
            }
        }

        res
    }
}
