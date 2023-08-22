// @leetup=info id=300 lang=cpp slug=longest-increasing-subsequence

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
  int lengthOfLIS(std::vector<int>& nums) {
    // dp version: O(N^2)
    // std::vector<int> dp(nums.size() + 1);
    // int res = 0;
    // for (size_t i = 0; i < nums.size(); ++i) {
    //   dp[i] = 1;
    //   for (size_t j = 0; j < i; ++j) {
    //     if (nums[j] < nums[i]) {
    //       dp[i] = std::max(dp[i], dp[j] + 1);
    //     }
    //   }
    //   res = std::max(res, dp[i]);
    // }

    // return res;

    // O(NlogN)
    // Ref:
    // https://leetcode.com/problems/longest-increasing-subsequence/solutions/74848/9-lines-c-code-with-o-nlogn-complexity/
    std::vector<int> r;
    for (int n : nums) {
      auto it = std::lower_bound(r.begin(), r.end(), n);
      if (it == r.end()) {
        r.push_back(n);
      } else {
        *it = n;
      }
    }
    return r.size();
  }
};
// @leetup=code.

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
