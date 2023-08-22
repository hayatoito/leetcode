// @leetup=info id=3 lang=cpp
// slug=longest-substring-without-repeating-characters

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <unordered_map>
#include <vector>

// @leetup=code

class Solution {
 public:
  int lengthOfLongestSubstring(std::string s) {
    std::unordered_map<char, int> prev;
    int left = 0;
    int result = 0;
    for (int i = 0; i < s.size(); ++i) {
      char c = s[i];
      auto found = prev.find(c);
      if (found != prev.end()) {
        left = std::max(left, found->second + 1);
        found->second = i;
      } else {
        prev[c] = i;
      }
      result = std::max(result, i + 1 - left);
    }
    return result;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
