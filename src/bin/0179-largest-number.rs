// @leetup=info id=179 lang=rust slug=largest-number

// @leetup=code

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.into_iter().map(|n| n.to_string()).collect::<Vec<_>>();
        nums.sort_by(|a, b| {
            let ab = format!("{}{}", a, b);
            let ba = format!("{}{}", b, a);
            ab.cmp(&ba).reverse()
        });
        let mut res = nums.concat();
        res = res.trim_start_matches("0").to_string();
        if res.is_empty() {
            "0".to_string()
        } else {
            res
        }
    }
}
// @leetup=code

struct Solution;

fn main() {
    // println!("{}", Solution::largest_number(vec![10, 2]));
    println!("res: {}", Solution::largest_number(vec![0, 0]));
}
