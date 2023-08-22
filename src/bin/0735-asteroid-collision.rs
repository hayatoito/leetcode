// @leetup=info id=735 lang=rust slug=asteroid-collision

// @leetup=code
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        for mut s in asteroids {
            while let Some(&last) = res.last() {
                if last < 0 || s >= 0 {
                    break;
                }
                if last == -s {
                    res.pop();
                    s = 0;
                    break;
                } else if last > -s {
                    s = 0;
                    break;
                } else {
                    res.pop();
                }
            }
            if s != 0 {
                res.push(s);
            }
        }
        res
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::asteroid_collision;
}
