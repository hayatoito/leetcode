// @leetup=info id=287 lang=rust slug=find-the-duplicate-number

// @leetup=code

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = nums[0] as usize;
        let mut fast = nums[0] as usize;
        // Find the cycle
        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
            if slow == fast {
                break;
            }
        }

        // Find the entrance to the cycle
        slow = nums[0] as usize;
        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[fast] as usize;
        }
        fast as i32
    }

    // Modifies the array, which does't meet the problem's requirement.
    // pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
    //     for i in 0..nums.len() {
    //         let n = nums[i].abs() as usize;
    //         if nums[n] < 0 {
    //             return n as i32;
    //         }
    //         nums[n] = -nums[n];
    //     }
    //     unreachable!()
    // }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::find_duplicate;
}
