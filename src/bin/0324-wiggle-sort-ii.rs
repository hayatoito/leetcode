// @leetup=info id=324 lang=rust slug=wiggle-sort-ii

// @leetup=code

use std::cmp::Ordering;

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let median = find_median(nums);

        // https://leetcode.com/problems/wiggle-sort-ii/solutions/77682/step-by-step-explanation-of-index-mapping-in-java/

        // Partition nums using virtual index.
        let mut left = 0;
        let mut right = nums.len();
        let mut i = 0;
        // [smaller,..., median, ...., x, x,..., larger, ...]
        //               left          i         right
        //
        // virtual_nums[..left] < median
        // virtual_nums[left..i] = median
        // virtual_nums[right..] > median

        // Example:
        // nums:  [4, 5, 5, 6]
        // median: 5
        // Mapped index: [2, 0, 3, 1]

        // vnums[5, 4, 6, 5]
        //       l,i       r
        // nums:  [4, 5, 5, 6]
        //   => nums[mapped_index[0]] = nums[2] = 5 == median
        //   => i += 1

        // vnums: [5, 4, 6, 5]
        //         l  i       r
        // nums:  [4, 5, 5, 6]
        //   => nums[mapped_index[1]] = nums[0] = 4 < median
        //   => Swap l and i

        // vnums: [4, 5, 6, 5]
        //            l  i    r
        // nums:  [[5], 5, [4], 6]
        //   => nums[mapped_index[2]] = nums[3] = 6 > median
        //   => Swap i and r-1

        // vnums: [4, 5, 5, 6]
        //            l  i  r
        // nums:  [5, [6], 4, [5]]
        //   => nums[mapped_index[2]] = nums[3] = 5 == median

        // vnums: [4, 5, 5, 6]
        //            l     i,r
        // nums:  [5, 6, 4, 5]

        while i < right {
            let m_i = mapped_index(i, nums.len());
            let m_left = mapped_index(left, nums.len());
            let m_right = mapped_index(right - 1, nums.len());
            match nums[m_i].cmp(&median) {
                Ordering::Less => {
                    nums.swap(m_i, m_left);
                    i += 1;
                    left += 1;
                }
                Ordering::Equal => i += 1,
                Ordering::Greater => {
                    nums.swap(m_i, m_right);
                    right -= 1;
                }
            }
        }
    }
}

fn find_median(nums: &mut [i32]) -> i32 {
    let m = (nums.len() - 1) / 2;
    partition_at_index(nums, m);
    nums[m]
}

fn mapped_index(i: usize, n: usize) -> usize {
    //  sorted:      a0 a1 a2 a3 a4 a5
    //  small half:  M     S     S
    //  Large half:     L     L     M

    //  sorted:      a0 a1 a2 a3
    //  small half:  M     S
    //  Large half:     L     M

    // n = 4
    // Array:        4    5    5    6
    //                  median
    // Original idx: 0    1    2    3
    // Mapped   idx: 2    0    3    1
    //
    // res:         [5,   6,   4,   5]

    // n = 7
    // Original idx: 0  1  2  3  4  5  6
    // Mapped   idx: 6  4  2  0  5  3  1

    let m = (n - 1) / 2;
    if i <= m {
        (m - i) * 2
    } else {
        1 + (n - 1 - i) * 2
    }
}

#[test]
fn new_index_test() {
    assert_eq!(
        (0..7).map(|i| mapped_index(i, 7)).collect::<Vec<_>>(),
        vec![6, 4, 2, 0, 5, 3, 1]
    );
    assert_eq!(
        (0..6).map(|i| mapped_index(i, 6)).collect::<Vec<_>>(),
        vec![4, 2, 0, 5, 3, 1]
    );
}

// [2023-07-04 Tue] If leetcode supports `partition_at_index`, use it.
fn partition_at_index(nums: &mut [i32], k: usize) {
    assert!(k < nums.len());
    let pivot = *nums.last().unwrap();
    let mut p = 0;
    for i in 0..nums.len() {
        if nums[i] <= pivot {
            nums.swap(p, i);
            p += 1;
        }
    }
    p -= 1;
    assert_eq!(nums[p], pivot);
    match k.cmp(&p) {
        Ordering::Less => partition_at_index(&mut nums[..p], k),
        Ordering::Equal => {}
        Ordering::Greater => partition_at_index(&mut nums[p..], k - p),
    }
}

#[test]
fn partition_at_index_test() {
    let a = vec![3, 1, 2, 5, 4];
    for i in 0..a.len() {
        let mut a = a.clone();
        partition_at_index(&mut a, i);
        assert!(a[..i].iter().all(|n| *n <= a[i]));
        assert!(a[i..].iter().all(|n| a[i] <= *n));
    }
}

// @leetup=code

struct Solution;

fn main() {
    let mut a = vec![1, 1, 2, 1, 2, 2, 1];
    Solution::wiggle_sort(&mut a);
    assert_eq!(a, vec![1, 2, 1, 2, 1, 2, 1]);

    let mut a = vec![4, 5, 5, 6];
    Solution::wiggle_sort(&mut a);
    assert_eq!(a, vec![5, 6, 4, 5]);
}
