// @leetup=info id=172 lang=rust slug=factorial-trailing-zeroes

// @leetup=code

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        // O(n):
        // let n = n as usize;
        // let mut n5 = vec![0; n + 1];
        // let mut c5 = 0;
        // for i in 1..(n + 1) {
        //     if i % 5 == 0 {
        //         n5[i] = n5[i / 5] + 1;
        //     }
        //     c5 += n5[i];
        // }
        // c5

        // O(logN):
        // Ref: https://leetcode.com/problems/factorial-trailing-zeroes/solutions/52470/4-lines-4ms-c-solution-with-explanations
        let mut cnt = 0;
        let mut i = 5;
        while n / i > 0 {
            cnt += n / i;
            i *= 5;
        }
        cnt
    }
}
// @leetup=code

struct Solution;

fn main() {
    println!("{}", Solution::trailing_zeroes(3));
    println!("{}", Solution::trailing_zeroes(5));
    println!("{}", Solution::trailing_zeroes(0));
}
