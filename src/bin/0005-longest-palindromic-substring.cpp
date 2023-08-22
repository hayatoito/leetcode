// @leetup=info id=5 lang=cpp slug=longest-palindromic-substring

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  size_t res_left = 0;
  size_t res_right = 0;
  std::string s;

  std::string longestPalindrome(std::string s) {
    this->s = s;
    for (size_t i = 0; i < s.size(); ++i) {
      check(i, i);
      check(i, i + 1);
    }
    return s.substr(res_left, res_right - res_left + 1);
  }

  void check(size_t left, size_t right) {
    while (0 <= left && right < s.size() && s[left] == s[right]) {
      if (res_right - res_left < right - left) {
        res_right = right;
        res_left = left;
      }
      left -= 1;
      right += 1;
    }
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
