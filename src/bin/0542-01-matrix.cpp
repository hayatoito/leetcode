// @leetup=info id=542 lang=cpp slug=01-matrix

#include <algorithm>
#include <cassert>
#include <cstddef>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  std::vector<std::vector<int>> updateMatrix(
      std::vector<std::vector<int>>& mat) {
    std::queue<std::tuple<size_t, size_t, size_t>> q;
    for (size_t r = 0; r < mat.size(); ++r) {
      for (size_t c = 0; c < mat[0].size(); ++c) {
        if (mat[r][c] == 0) {
          q.push({r, c, 0});
        }
      }
    }

    std::vector<std::vector<int>> res(mat.size(),
                                      std::vector<int>(mat[0].size(), -1));

    while (!q.empty()) {
      auto [r, c, dist] = q.front();
      q.pop();
      if (r < mat.size() && c < mat[0].size() && res[r][c] == -1) {
        res[r][c] = dist;
        q.push({r, c + 1, dist + 1});
        q.push({r, c - 1, dist + 1});
        q.push({r + 1, c, dist + 1});
        q.push({r - 1, c, dist + 1});
      }
    }

    return res;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
