// @leetup=info id=315 lang=rust slug=count-of-smaller-numbers-after-self

// @leetup=code

// Refs:
// https://leetcode.com/problems/count-of-smaller-numbers-after-self/solutions/445769/merge-sort-clear-simple-explanation-with-examples-o-n-lg-n/
// https://leetcode.com/problems/count-of-smaller-numbers-after-self/solutions/2324770/elixir-fenwick-tree-rust-mergesort-solutions/

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        assert!(!nums.is_empty());
        let mut res = vec![0; nums.len()];
        let nums = nums.into_iter().enumerate().collect::<Vec<_>>();
        merge_sort(&nums[..], &mut res);
        res
    }
}

fn merge_sort(nums: &[(usize, i32)], res: &mut Vec<i32>) -> Vec<(usize, i32)> {
    if nums.len() > 1 {
        let m = nums.len() / 2;
        let a = merge_sort(&nums[..m], res);
        let b = merge_sort(&nums[m..], res);
        merge(a, b, res)
    } else {
        nums.into_iter().cloned().collect()
    }
}

fn merge(a: Vec<(usize, i32)>, b: Vec<(usize, i32)>, res: &mut Vec<i32>) -> Vec<(usize, i32)> {
    let mut merged = Vec::with_capacity(a.len() + b.len());
    let mut i = 0;
    let mut j = 0;
    let mut cnt = 0;

    while i < a.len() && j < b.len() {
        if a[i].1 <= b[j].1 {
            res[a[i].0] += cnt;
            merged.push(a[i]);
            i += 1;
        } else {
            cnt += 1;
            merged.push(b[j]);
            j += 1;
        }
    }

    for n in &a[i..] {
        res[n.0] += cnt;
    }
    merged.extend_from_slice(&a[i..]);
    merged.extend_from_slice(&b[j..]);
    merged
}

// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::count_smaller;
}
