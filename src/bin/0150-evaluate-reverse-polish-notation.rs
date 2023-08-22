// @leetup=info id=150 lang=rust slug=evaluate-reverse-polish-notation

// @leetup=code
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut s = Vec::<i32>::new();
        for a in tokens {
            match a.as_str() {
                "+" | "-" | "*" | "/" => {
                    let a2 = s.pop().unwrap();
                    let a1 = s.pop().unwrap();
                    match a.as_str() {
                        "+" => {
                            s.push(a1 + a2);
                        }
                        "-" => {
                            s.push(a1 - a2);
                        }
                        "*" => {
                            s.push(a1 * a2);
                        }
                        "/" => {
                            s.push(a1 / a2);
                        }
                        _ => unreachable!(),
                    }
                }
                a => s.push(a.parse().unwrap()),
            }
        }

        assert_eq!(s.len(), 1);
        s[0]
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::eval_rpn;
}
