// @leetup=info id=200 lang=cpp slug=number-of-islands

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
  int numIslands(std::vector<std::vector<char>>& grid) {
    int cnt = 0;
    for (size_t r = 0; r < grid.size(); ++r) {
      for (size_t c = 0; c < grid[0].size(); ++c) {
        if (grid[r][c] == '1') {
          ++cnt;
          dfs(r, c, grid);
        }
      }
    }
    return cnt;
  }

  void dfs(size_t r, size_t c, std::vector<std::vector<char>>& grid) {
    if (r < grid.size() && c < grid[0].size() && grid[r][c] == '1') {
      grid[r][c] = '0';
      dfs(r, c + 1, grid);
      dfs(r, c - 1, grid);
      dfs(r + 1, c, grid);
      dfs(r - 1, c, grid);
    }
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
