// @leetup=info id=344 lang=rust slug=reverse-string

// @leetup=code

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();
        for i in 0..len {
            if i >= len - 1 - i {
                return;
            }
            s.swap(i, len - 1 - i);
        }
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::reverse_string;
}
