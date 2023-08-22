// @leetup=info id=17 lang=cpp slug=letter-combinations-of-a-phone-number

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
  std::vector<std::string> res;
  std::string s;
  std::string digits;
  std::vector<std::string> d_to_chars;

  std::vector<std::string> letterCombinations(std::string digits) {
    if (digits.empty()) {
      return {};
    }

    this->digits = digits;
    d_to_chars = {"",    "",    "abc",  "def", "ghi",
                  "jkl", "mno", "pqrs", "tuv", "wxyz"};
    assert(d_to_chars.size() == 10);

    dfs(0);
    return res;
  }

  void dfs(size_t i) {
    if (i == digits.size()) {
      res.push_back(s);
      return;
    }
    for (char c : d_to_chars[digits[i] - '0']) {
      s.push_back(c);
      dfs(i + 1);
      s.pop_back();
    }
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
