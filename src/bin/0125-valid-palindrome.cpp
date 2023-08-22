// @leetup=info id=125 lang=cpp slug=valid-palindrome

#include <algorithm>
#include <cctype>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

#include <cctype>

class Solution {
 public:
  bool isPalindrome(std::string s) {
    // std::string t;
    // for (char c : s) {
    //   if (std::isalnum(c)) {
    //     t += std::tolower(c);
    //   }
    // }

    s.erase(std::remove_if(s.begin(), s.end(),
                           [](char c) { return !std::isalnum(c); }),
            s.end());
    std::transform(s.begin(), s.end(), s.begin(),
                   [](char c) { return std::tolower(c); });

    int left = 0;
    int right = s.size() - 1;
    while (left < right) {
      if (s[left] != s[right]) {
        return false;
      }
      left += 1;
      right -= 1;
    }
    return true;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
