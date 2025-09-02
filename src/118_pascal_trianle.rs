pub struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle: Vec<Vec<i32>> = Vec::new();

        for i in 0..num_rows as usize {
            let mut temp: Vec<i32> = vec![1; i + 1];

            for j in 1..i {
                temp[j] = triangle[i - 1][j - 1] + triangle[i - 1][j];
            }
            triangle.push(temp)
        }
        triangle
    }
}

fn main() {
    // assert_eq!(
    //     Solution::generate(5),
    //     vec![[1], [1, 1], [1, 2, 1], [1, 3, 3, 1], [1, 4, 6, 4, 1]]
    // )

    println!("{:?}", Solution::generate(5));
}
