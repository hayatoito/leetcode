// @leetup=info id=171 lang=rust slug=excel-sheet-column-number

// @leetup=code

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
            .chars()
            .fold(0, |acc, c| acc * 26 + (c as i32 - 'A' as i32 + 1))
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::title_to_number;
}
