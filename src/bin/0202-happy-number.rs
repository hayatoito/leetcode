// @leetup=info id=202 lang=rust slug=happy-number

// @leetup=code

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = n;
        loop {
            slow = Solution::next(slow);
            fast = Solution::next(Solution::next(fast));
            if slow == 1 || fast == 1 {
                return true;
            }
            if slow == fast {
                return false;
            }
        }
    }

    fn next(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            let rem = n % 10;
            res += rem * rem;
            n /= 10;
        }
        res
    }
}
// @leetup=code

struct Solution;

fn main() {
    println!("19: {}", Solution::is_happy(19));
}
