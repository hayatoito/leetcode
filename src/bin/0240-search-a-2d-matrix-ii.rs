// @leetup=info id=240 lang=rust slug=search-a-2d-matrix-ii

// @leetup=code

use std::cmp::Ordering;

impl Solution {

    // O(R + C)
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut r = 0;
        let mut c = matrix[0].len() - 1;
        while r < matrix.len() {
            match matrix[r][c].cmp(&target) {
                Ordering::Less => {
                    r += 1;
                }
                Ordering::Equal => {
                    return true;
                }
                Ordering::Greater => {
                    if c == 0 {
                        return false;
                    }
                    c -= 1;
                }
            }
        }
        false
    }

    // Devide and Conquer: O(NlogN)?

    // pub fn search_matrix2(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    //     Solution::search(&matrix, 0, matrix.len(), 0, matrix[0].len(), target)
    // }

    // fn search(m: &[Vec<i32>], r1: usize, r2: usize, c1: usize, c2: usize, target: i32) -> bool {
    //     if r1 == r2 || c1 == c2 {
    //         return false;
    //     }
    //     if r2 - r1 <= 2 && c2 - c1 <= 2 {
    //         for r in r1..r2 {
    //             for c in c1..c2 {
    //                 if m[r][c] == target {
    //                     return true;
    //                 }
    //             }
    //         }
    //         return false;
    //     }
    //     let mid_r = r1 + (r2 - r1) / 2;
    //     let mid_c = c1 + (c2 - c1) / 2;
    //     match target.cmp(&m[mid_r][mid_c]) {
    //         Ordering::Equal => true,
    //         Ordering::Less => {
    //             Solution::search(m, r1, mid_r + 1, c1, mid_c + 1, target)
    //                 || Solution::search(m, r1, mid_r, mid_c + 1, c2, target)
    //                 || Solution::search(m, mid_r + 1, r2, c1, mid_c, target)
    //         }
    //         Ordering::Greater => {
    //             Solution::search(m, mid_r, r2, mid_c, c2, target)
    //                 || Solution::search(m, r1, mid_r, mid_c + 1, c2, target)
    //                 || Solution::search(m, mid_r + 1, r2, c1, mid_c, target)
    //         }
    //     }
    // }
}
// @leetup=code

struct Solution;

fn main() {
    println!("{}", Solution::search_matrix(vec![vec![1, 3, 5]], 4));
}
