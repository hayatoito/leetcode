// @leetup=info id=739 lang=cpp slug=daily-temperatures

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
  std::vector<int> dailyTemperatures(std::vector<int>& temperatures) {
    std::stack<size_t> stack;
    std::vector<int> res(temperatures.size());
    for (size_t i = 0; i < temperatures.size(); ++i) {
      int n = temperatures[i];
      while (!stack.empty()) {
        size_t prev = stack.top();
        if (temperatures[prev] >= n) {
          break;
        }
        res[prev] = (i - prev);
        stack.pop();
      }
      stack.push(i);
    }
    return res;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
