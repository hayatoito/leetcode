// @leetup=info id=224 lang=cpp slug=basic-calculator

#include <algorithm>
#include <cassert>
#include <cctype>
#include <iostream>
#include <map>
#include <set>
#include <stack>
#include <string>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  int calculate(std::string s) {
    int res = 0;
    std::stack<int> res_stack;
    std::stack<int> sign_stack;
    int sign = 1;
    int num = 0;
    for (char c : s) {
      if (std::isdigit(c)) {
        num = num * 10 + (c - '0');
      } else if (c == '+') {
        res += num * sign;
        num = 0;
        sign = 1;
      } else if (c == '-') {
        res += num * sign;
        num = 0;
        sign = -1;
      } else if (c == '(') {
        assert(num == 0);
        res_stack.push(res);
        sign_stack.push(sign);
        res = 0;
        sign = 1;
      } else if (c == ')') {
        res = (res + num * sign) * sign_stack.top() + res_stack.top();
        res_stack.pop();
        sign_stack.pop();

        num = 0;
        sign = 1;
      } else {
        // Skip whitespace.
      }
    }
    return res + num * sign;
  }
};

// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
