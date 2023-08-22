// @leetup=info id=140 lang=rust slug=word-break-ii

// @leetup=code
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut res = Vec::new();
        Solution::solve(&word_dict, s.as_str(), &mut Vec::new(), &mut res);
        res
    }

    fn solve(word_dict: &[String], s: &str, v: &mut Vec<String>, res: &mut Vec<String>) {
        if s.is_empty() {
            res.push(v.join(" "));
            return;
        }
        for substr in word_dict {
            if s.starts_with(substr) {
                v.push(substr.clone());
                Solution::solve(word_dict, &s[substr.len()..], v, res);
                v.pop();
            }
        }
    }
}

// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::word_break;
}
