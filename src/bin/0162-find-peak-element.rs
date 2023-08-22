// @leetup=info id=162 lang=rust slug=find-peak-element

// @leetup=code

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        Solution::find(&nums[..]) as i32
    }

    fn find(nums: &[i32]) -> usize {
        if nums.len() == 1 {
            0
        } else {
            let mid = nums.len() / 2;
            if nums[mid - 1] < nums[mid] {
                mid + Solution::find(&nums[mid..])
            } else {
                Solution::find(&nums[..mid])
            }
        }
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::find_peak_element;
}
