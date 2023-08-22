// @leetup=info id=289 lang=rust slug=game-of-life

// @leetup=code

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                let mut live_cnt = 0;
                for dr in [!0, 0, 1] {
                    for dc in [!0, 0, 1] {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let nr = r.wrapping_add(dr);
                        let nc = c.wrapping_add(dc);
                        if nr < board.len() && nc < board[0].len() && board[nr][nc] & 1 == 1 {
                            live_cnt += 1;
                        }
                    }
                }
                if board[r][c] & 1 == 1 && (live_cnt == 2 || live_cnt == 3) {
                    board[r][c] |= 1 << 1;
                }
                if board[r][c] & 1 == 0 && live_cnt == 3 {
                    board[r][c] |= 1 << 1;
                }
            }
        }
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                board[r][c] = board[r][c] >> 1;
            }
        }
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::game_of_life;
}
