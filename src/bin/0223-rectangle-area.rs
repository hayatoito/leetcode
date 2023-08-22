// @leetup=info id=223 lang=rust slug=rectangle-area

// @leetup=code
impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {

        fn interval_intersection(seg1: (i32, i32), seg2: (i32, i32)) -> i32 {
            let left = seg1.0.max(seg2.0);
            let right = seg1.1.min(seg2.1);
            if left < right {
                right - left
            } else {
                0
            }
        }

        let area_a = (ax2 - ax1) * (ay2 - ay1);
        let area_b = (bx2 - bx1) * (by2 - by1);
        let seg1 = interval_intersection((ax1, ax2), (bx1, bx2));
        let seg2 = interval_intersection((ay1, ay2), (by1, by2));
        let area_intersection = seg1 * seg2;
        area_a + area_b - area_intersection
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::compute_area;
}
