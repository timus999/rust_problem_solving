pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let merged: Vec<i32> = vec![];

        let (mut i, mut j, mut k) = (0, 0, 0);

        while i < nums1.len() && j < nums2.len() {
            if nums[i] < nums2[j] {
                merged[k].push(nums[i]);
                k += 1;
                i += 1;
            } else {
                merged[k].push(nums[j]);
                k += 1;
                j += 1;
            }
        }

        while i < nums1.len() {
            merged[k].push(nums1[i]);
            k += 1;
            i += 1;
        }

        while j < nums2.len() {
            merged[k].push(nums2[j]);
            k += 1;
            j += 1;
        }
        if merged.len() % 2 == 1 {
            return merged[merged.len() / 2] as f64;
        } else {
            return ((merged[merged.len() / 2] + merged[merged.len() / 2 - 1]) / 2) as f64;
        }
    }
}

fn main() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.5
    );
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2);
}
