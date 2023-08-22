// @leetup=info id=75 lang=rust slug=sort-colors

// @leetup=code

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let left = Solution::sort(nums, 0);
        Solution::sort(&mut nums[left..], 1);
    }

    fn sort(nums: &mut [i32], x: i32) -> usize {
        let mut left = 0;
        for i in 0..nums.len() {
            if nums[i] == x {
                nums.swap(left, i);
                left += 1;
            }
        }
        left
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::sort_colors;
}
