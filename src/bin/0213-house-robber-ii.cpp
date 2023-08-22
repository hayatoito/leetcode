// @leetup=info id=213 lang=cpp slug=house-robber-ii

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
    return std::max(rob2(nums.begin(), std::prev(nums.end())),
                    rob2(std::next(nums.begin()), nums.end()));
  }

  int rob2(std::vector<int>::iterator begin, std::vector<int>::iterator end) {
    int dp1 = 0;
    int dp2 = 0;
    while (begin != end) {
      int tmp = dp1;
      dp1 = std::max(dp1, dp2);
      dp2 = tmp + *begin;
      ++begin;
    }
    return std::max(dp1, dp2);
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
