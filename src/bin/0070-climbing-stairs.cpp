// @leetup=info id=70 lang=cpp slug=climbing-stairs

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
  int climbStairs(int n) {
    int dp1 = 1;
    int dp2 = 1;
    for (int i = 2; i <= n; ++i) {
      int tmp = dp2;
      dp2 = dp1 + dp2;
      dp1 = tmp;
    }
    return dp2;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
