use std::collections::HashSet;

pub struct Solution {}

// impl Solution {
//     pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
//         let n: usize = nums.len();

//         let mut res: HashSet<Vec<i32>> = HashSet::new();

//         for i in 0..n {
//             let mut seen: HashSet<i32> = HashSet::new();
//             for j in i + 1..n {
//                 let k = -(nums[i] + nums[j]);

//                 if (seen.contains(&k)) {
//                     let mut triplets = vec![nums[i], nums[j], k];
//                     triplets.sort();
//                     res.insert(triplets);
//                 } else {
//                     seen.insert(nums[j]);
//                 }
//             }
//         }

//         res.into_iter().collect()
//     }
// }
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let n = nums.len();
        let mut res = Vec::new();

        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut j = i + 1;
            let mut k = n - 1;

            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == 0 {
                    res.push(vec![nums[i], nums[j], nums[k]]);

                    while j < k && nums[j] == nums[j + 1] {
                        j += 1;
                    }
                    while j < k && nums[k] == nums[k - 1] {
                        k -= 1;
                    }

                    j += 1;
                    k -= 1;
                } else if sum < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }

        res
    }
}
