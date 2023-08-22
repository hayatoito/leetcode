// @leetup=info id=733 lang=rust slug=flood-fill

// @leetup=code
impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let src_color = image[sr as usize][sc as usize];
        if src_color == color {
            return image;
        }

        fn dfs(r: usize, c: usize, image: &mut Vec<Vec<i32>>, src_color: i32, color: i32) {
            if r < image.len() && c < image[0].len() && image[r][c] == src_color {
                image[r][c] = color;
                dfs(r, c + 1, image, src_color, color);
                dfs(r, c.wrapping_sub(1), image, src_color, color);
                dfs(r + 1, c, image, src_color, color);
                dfs(r.wrapping_sub(1), c, image, src_color, color);
            }
        }
        dfs(sr as usize, sc as usize, &mut image, src_color, color);
        image
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::flood_fill;
}
