// @leetup=info id=91 lang=cpp slug=decode-ways

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  int numDecodings(std::string s) {
    std::set<std::string> valid;
    for (size_t i = 1; i <= 26; ++i) {
      valid.insert(std::to_string(i));
    }

    std::vector<int> dp(s.size() + 1);
    dp[0] = 1;
    for (size_t i = 0; i < s.size() + 1; ++i) {
      for (size_t j = 0; j < i; j++) {
        if (valid.find(s.substr(j, i - j)) != valid.end()) {
          dp[i] += dp[j];
        }
      }
    }
    return dp[s.size()];
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
