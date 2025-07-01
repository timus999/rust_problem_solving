pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let table: Vec<Vec<char>> = vec![Vec::new(), num_rows];

        let chars: Vec<char> = s.chars().collect();

        let mut count = 0;

        let mut direction = true;
        for c in chars {
            if count >= num_rows {
                direction = !direction;
            } else if count < 0 {
                direction = !direction;
                count = 0;
            }
            if direction {
                table[count].push(c);
                count += 1;
            } else {
                count -= 1;
                table[count].push(c);
            }
        }

        println!("{:?}", table);
    }
}
