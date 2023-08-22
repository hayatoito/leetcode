// @leetup=info id=200 lang=rust slug=number-of-islands

// @leetup=code
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        fn dfs(r: usize, c: usize, grid: &mut Vec<Vec<char>>) {
            if r < grid.len() && c < grid[0].len() && grid[r][c] == '1' {
                grid[r][c] = '0';
                dfs(r + 1, c, grid);
                dfs(r.wrapping_sub(1), c, grid);
                dfs(r, c + 1, grid);
                dfs(r, c.wrapping_sub(1), grid);
            }
        }

        let mut cnt = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == '1' {
                    cnt += 1;
                    dfs(r, c, &mut grid);
                }
            }
        }
        cnt
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::num_islands;
}
