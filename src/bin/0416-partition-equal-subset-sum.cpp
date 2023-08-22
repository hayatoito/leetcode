// @leetup=info id=416 lang=cpp slug=partition-equal-subset-sum

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <numeric>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  bool canPartition(std::vector<int>& nums) {
    int sum = std::accumulate(nums.begin(), nums.end(), 0);
    if (sum % 2 == 1) {
      return false;
    }
    std::vector<bool> dp(sum + 1);
    dp[0] = true;
    for (int num : nums) {
      for (size_t i = dp.size() - 1; i < dp.size(); --i) {
        if (dp[i]) {
          dp[i + num] = true;
        }
      }
    }
    return dp[sum / 2];
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
