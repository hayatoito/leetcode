// @leetup=info id=37 lang=cpp slug=sudoku-solver

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  size_t N = 9;

  void solveSudoku(std::vector<std::vector<char>>& board) {
    assert(dfs(0, board));
  }

  bool dfs(size_t i, std::vector<std::vector<char>>& board) {
    if (i == N * N) {
      return true;
    }
    size_t r = i / N;
    size_t c = i % N;
    if (board[r][c] != '.') {
      return dfs(i + 1, board);
    }
    for (char n = '1'; n <= '9'; ++n) {
      if (is_valid(r, c, n, board)) {
        board[r][c] = n;
        if (dfs(i + 1, board)) {
          return true;
        }
        board[r][c] = '.';
      }
    }
    return false;
  }

  bool is_valid(size_t r,
                size_t c,
                char n,
                std::vector<std::vector<char>>& board) {
    for (size_t i = 0; i < N; ++i) {
      if (board[r][i] == n) {
        return false;
      }
      if (board[i][c] == n) {
        return false;
      }
    }

    size_t start_r = r - (r % 3);
    size_t start_c = c - (c % 3);
    for (size_t i = start_r; i < start_r + 3; ++i) {
      for (size_t j = start_c; j < start_c + 3; ++j) {
        if (board[i][j] == n) {
          // std::cout << "False: r: " << r << ", c: " << c << ", n: " << n <<
          // std::endl;
          return false;
        }
      }
    }
    return true;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
