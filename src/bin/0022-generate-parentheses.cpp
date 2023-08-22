// @leetup=info id=22 lang=cpp slug=generate-parentheses

#include <algorithm>
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

  std::vector<std::string> generateParenthesis(int n) {
    gen(n, n);
    return res;
  }

  void gen(int open_paren, int close_paren) {
    if (open_paren == 0 && close_paren == 0) {
      res.push_back(s);
      return;
    }

    if (open_paren != 0) {
      s.push_back('(');
      gen(open_paren - 1, close_paren);
      s.pop_back();
    }
    if (open_paren < close_paren) {
      s.push_back(')');
      gen(open_paren, close_paren - 1);
      s.pop_back();
    }
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
