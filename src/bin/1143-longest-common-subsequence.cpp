// @leetup=info id=1143 lang=cpp slug=longest-common-subsequence

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
  int longestCommonSubsequence(std::string text1, std::string text2) {
    std::vector<int> dp1(text2.size() + 1);
    for (char c : text1) {
      std::vector<int> dp2(text2.size() + 1);
      for (size_t i = 0; i < text2.size(); ++i) {
        if (c == text2[i]) {
          dp2[i + 1] = dp1[i] + 1;
        } else {
          dp2[i + 1] = std::max(dp2[i], dp1[i + 1]);
        }
      }
      dp1 = std::move(dp2);
    }
    return dp1[text2.size()];
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
