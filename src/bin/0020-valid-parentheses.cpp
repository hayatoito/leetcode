// @leetup=info id=20 lang=cpp slug=valid-parentheses

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <stack>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  bool isValid(std::string s) {
    std::stack<char> stack;
    std::map<char, char> pairs{{')', '('}, {'}', '{'}, {']', '['}};

    for (char c : s) {
      auto it = pairs.find(c);
      if (it != pairs.end()) {
        if (stack.empty()) {
          return false;
        }
        char top = stack.top();
        stack.pop();
        if (it->second != top) {
          return false;
        }
      } else {
        stack.push(c);
      }
    }
    return stack.empty();
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
