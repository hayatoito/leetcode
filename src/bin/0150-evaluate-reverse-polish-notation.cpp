// @leetup=info id=150 lang=cpp slug=evaluate-reverse-polish-notation

#include <algorithm>
#include <cassert>
#include <functional>
#include <iostream>
#include <map>
#include <set>
#include <stack>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  int evalRPN(std::vector<std::string>& tokens) {
    std::stack<int> stack;
    for (std::string& s : tokens) {
      std::function<int(int, int)> op;
      if (s == "+") {
        op = std::plus<int>();
      } else if (s == "-") {
        op = std::minus<int>();
      } else if (s == "*") {
        op = std::multiplies<int>();
      } else if (s == "/") {
        op = std::divides<int>();
      }
      if (op) {
        int a1 = stack.top();
        stack.pop();
        int a2 = stack.top();
        stack.pop();
        stack.push(op(a2, a1));
      } else {
        stack.push(std::stoi(s));
      }
    }
    assert(stack.size() == 1);
    return stack.top();
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
