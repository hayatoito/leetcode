// @leetup=info id=139 lang=cpp slug=word-break

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
  bool wordBreak(std::string s, std::vector<std::string>& wordDict) {
    std::set<std::string> dict(wordDict.begin(), wordDict.end());
    size_t N = s.size();
    std::vector<bool> dp(N + 1);
    dp[0] = true;
    for (size_t end = 1; end <= N; ++end) {
      for (size_t start = 0; start < end; ++start) {
        if (dp[start] &&
            dict.find(s.substr(start, end - start)) != dict.end()) {
          dp[end] = true;
          break;
        }
      }
    }
    return dp[N];
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
