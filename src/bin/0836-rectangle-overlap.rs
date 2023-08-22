// @leetup=info id=836 lang=rust slug=rectangle-overlap

// @leetup=code
impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {

        fn is_segment_overlap(seg1: (i32, i32), seg2: (i32, i32)) -> bool {
            seg1.0 < seg2.1 && seg2.0 < seg1.1
        }

        is_segment_overlap((rec1[0], rec1[2]), (rec2[0], rec2[2])) &&
            is_segment_overlap((rec1[1], rec1[3]), (rec2[1], rec2[3]))
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::is_rectangle_overlap;
}
