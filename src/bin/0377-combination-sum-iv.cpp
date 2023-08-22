// @leetup=info id=377 lang=cpp slug=combination-sum-iv

#include <algorithm>
#include <cassert>
#include <cstdint>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  int combinationSum4(std::vector<int>& nums, int target) {
    std::vector<size_t> dp(target + 1);
    dp[0] = 1;
    for (int i = 0; i < target; ++i) {
      for (int n : nums) {
        if (i + n <= target) {
          dp[i + n] += dp[i];
        }
      }
    }
    return static_cast<int>(dp[target]);
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
