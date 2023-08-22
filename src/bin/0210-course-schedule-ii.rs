// @leetup=info id=210 lang=rust slug=course-schedule-ii

// @leetup=code

use std::collections::*;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut graph = vec![vec![]; num_courses];
        let mut in_degree = vec![0; num_courses];

        for pre in prerequisites {
            let (n, pre) = (pre[0] as usize, pre[1] as usize);
            graph[pre].push(n);
            in_degree[n] += 1
        }

        let mut q = VecDeque::new();
        for (i, &n) in in_degree.iter().enumerate() {
            if n == 0 {
                q.push_back(i);
            }
        }

        let mut visited = HashSet::new();

        let mut res = Vec::new();
        while let Some(a) = q.pop_front() {
            if visited.insert(a) {
                res.push(a as i32);
                for &b in &graph[a] {
                    in_degree[b] -= 1;
                    if in_degree[b] == 0 {
                        q.push_back(b);
                    }
                }
            }
        }

        if num_courses == visited.len() {
            res
        } else {
            Vec::new()
        }
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::find_order;
}
