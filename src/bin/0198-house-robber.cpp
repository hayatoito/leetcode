// @leetup=info id=198 lang=cpp slug=house-robber

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
  int rob(std::vector<int>& nums) {
    if (nums.size() == 1) {
      return nums[0];
    }
    int dp1 = nums[0];
    int dp2 = nums[1];

    for (size_t i = 2; i < nums.size(); ++i) {
      int tmp = dp1;
      dp1 = std::max(dp1, dp2);
      dp2 = tmp + nums[i];
    }

    return std::max(dp1, dp2);
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
