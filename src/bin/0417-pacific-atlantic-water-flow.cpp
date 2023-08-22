// @leetup=info id=417 lang=cpp slug=pacific-atlantic-water-flow

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
  std::vector<std::vector<int>> pacificAtlantic(
      std::vector<std::vector<int>>& heights) {
    R = heights.size();
    C = heights[0].size();

    std::queue<std::tuple<size_t, size_t, int>> q1;
    for (size_t r = 0; r < R; ++r) {
      q1.push({r, 0, 0});
    }
    for (size_t c = 0; c < C; ++c) {
      q1.push({0, c, 0});
    }
    std::set<std::tuple<size_t, size_t>> visited1 =
        fill(std::move(q1), heights);

    std::queue<std::tuple<size_t, size_t, int>> q2;
    for (size_t r = 0; r < R; ++r) {
      q2.push({r, C - 1, 0});
    }
    for (size_t c = 0; c < C; ++c) {
      q2.push({R - 1, c, 0});
    }
    std::set<std::tuple<size_t, size_t>> visited2 =
        fill(std::move(q2), heights);

    std::set<std::tuple<size_t, size_t>> visited;
    std::set_intersection(visited1.begin(), visited1.end(), visited2.begin(),
                          visited2.end(),
                          std::inserter(visited, visited.end()));

    std::vector<std::vector<int>> res;
    for (auto [r, c] : visited) {
      res.push_back({static_cast<int>(r), static_cast<int>(c)});
    }
    return res;
  }

 private:
  std::set<std::tuple<size_t, size_t>> fill(
      std::queue<std::tuple<size_t, size_t, int>> q,
      const std::vector<std::vector<int>>& heights) {
    std::set<std::tuple<size_t, size_t>> visited;
    while (!q.empty()) {
      auto [r, c, height] = q.front();
      q.pop();
      if (r < R && c < C && height <= heights[r][c] &&
          visited.insert({r, c}).second) {
        q.push({r, c + 1, heights[r][c]});
        q.push({r, c - 1, heights[r][c]});
        q.push({r + 1, c, heights[r][c]});
        q.push({r - 1, c, heights[r][c]});
      }
    }
    return visited;
  }

  size_t R;
  size_t C;
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
