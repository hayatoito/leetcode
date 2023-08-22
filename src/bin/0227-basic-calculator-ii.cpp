// @leetup=info id=227 lang=cpp slug=basic-calculator-ii

#include <algorithm>
#include <cassert>
#include <cctype>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  int calculate(std::string s) {
    int res = 0;
    int term = 1;
    char term_op = '*';
    int num = 0;
    // Sentinal;
    s += "+";
    for (char c : s) {
      if (c == ' ') {
        continue;
      }
      if (std::isdigit(c)) {
        num = num * 10 + (c - '0');
        continue;
      }
      if (c == '+' || c == '-') {
        // term * [term_op] num + [c]
        if (term_op == '*') {
          res += term * num;
        } else if (term_op == '/') {
          res += term / num;
        }
        if (c == '+') {
          term = 1;
        } else {
          term = -1;
        }
        num = 0;
        term_op = '*';
      } else if (c == '*' || c == '/') {
        if (term_op == '*') {
          term *= num;
        } else if (term_op == '/') {
          term /= num;
        }
        num = 0;
        term_op = c;
      }
    }
    return res;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
