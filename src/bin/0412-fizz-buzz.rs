// @leetup=info id=412 lang=rust slug=fizz-buzz

// @leetup=code

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        for i in 1..(n + 1) {
            if i % 3 == 0 && i % 5 == 0 {
                res.push("FizzBuzz".to_owned());
            } else if i % 3 == 0 {
                res.push("Fizz".to_owned());
            } else if i % 5 == 0 {
                res.push("Buzz".to_owned());
            } else {
                res.push(i.to_string());
            }
        }
        res
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::fizz_buzz;
}
