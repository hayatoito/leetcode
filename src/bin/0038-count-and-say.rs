// @leetup=info id=38 lang=rust slug=count-and-say

// @leetup=code
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut a = vec![1];
        for _ in 1..n {
            let mut b = vec![];
            let mut slice = &a[..];
            while !slice.is_empty() {
                let c = slice[0];
                let count = slice.iter().take_while(|i| **i == c).count();
                assert!(count < 10);
                b.push(count);
                b.push(c);
                slice = &slice[count..];
            }
            a = b;
        }
        a.into_iter().map(|i| i.to_string()).collect()
    }
}
// @leetup=code

struct Solution;

fn main() {
    for n in 1..31 {
        println!("n: {}, say: {}", n, Solution::count_and_say(n));
    }
}
