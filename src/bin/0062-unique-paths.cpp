// @leetup=info id=62 lang=cpp slug=unique-paths

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
  int uniquePaths(int m, int n) {
    std::vector<int> dp1(n + 1, 1);
    for (int i = 1; i < m; ++i) {
      std::vector<int> dp2(n + 1, 0);
      for (int j = 0; j < n; ++j) {
        dp2[j + 1] = dp2[j] + dp1[j + 1];
      }
      std::swap(dp1, dp2);
    }
    return dp1[n];
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
