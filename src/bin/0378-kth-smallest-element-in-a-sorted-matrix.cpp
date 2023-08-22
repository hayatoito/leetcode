// @leetup=info id=378 lang=cpp slug=kth-smallest-element-in-a-sorted-matrix

#include <algorithm>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  int kthSmallest(std::vector<std::vector<int>>& matrix, int k) {
    std::priority_queue<std::tuple<int, size_t, size_t>> q;

    q.push({-matrix[0][0], 0, 0});
    int res;
    for (int i = 0; i < k; ++i) {
      auto [n, r, c] = q.top();
      res = -n;
      q.pop();
      if (c == 0 && r + 1 < matrix.size()) {
        q.push({-matrix[r + 1][c], r + 1, c});
      }
      if (c + 1 < matrix[0].size()) {
        q.push({-matrix[r][c + 1], r, c + 1});
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
