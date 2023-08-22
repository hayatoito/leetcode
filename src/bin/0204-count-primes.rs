// @leetup=info id=204 lang=rust slug=count-primes

// @leetup=code

// Sieve of eratothons.
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }
        let n = n as usize;
        let sqrt_n = (n as f64).sqrt() as usize;
        let mut is_prime = vec![true; n];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..=sqrt_n {
            if is_prime[i] {
                for j in (i * i..n).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }
        is_prime.into_iter().filter(|&a| a).count() as i32
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::count_primes;
}
