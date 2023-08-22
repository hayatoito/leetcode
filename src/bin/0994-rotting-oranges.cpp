// @leetup=info id=994 lang=cpp slug=rotting-oranges

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  int orangesRotting(std::vector<std::vector<int>>& grid) {
    size_t fresh_cnt = 0;

    std::queue<std::tuple<size_t, size_t, size_t>> q;
    for (size_t r = 0; r < grid.size(); ++r) {
      for (size_t c = 0; c < grid[0].size(); ++c) {
        if (grid[r][c] == 1) {
          ++fresh_cnt;
        } else if (grid[r][c] == 2) {
          q.push({r, c + 1, 1});
          q.push({r, c - 1, 1});
          q.push({r + 1, c, 1});
          q.push({r - 1, c, 1});
        }
      }
    }

    int res = 0;

    while (!q.empty()) {
      auto [r, c, dist] = q.front();
      q.pop();
      if (r < grid.size() && c < grid[0].size() && grid[r][c] == 1) {
        --fresh_cnt;
        res = dist;
        grid[r][c] = 2;
        q.push({r, c + 1, dist + 1});
        q.push({r, c - 1, dist + 1});
        q.push({r + 1, c, dist + 1});
        q.push({r - 1, c, dist + 1});
      }
    }
    if (fresh_cnt == 0) {
      return res;
    }
    return -1;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
