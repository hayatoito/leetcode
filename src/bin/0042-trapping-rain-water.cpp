// @leetup=info id=42 lang=cpp slug=trapping-rain-water

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
  int trap(std::vector<int>& height) {
    // Monotonic stack (descending height[index] order).
    // height[stack[i]] > height[stack[i + 1]] > ....
    std::stack<size_t> stack;
    int res = 0;
    for (size_t i = 0; i < height.size(); ++i) {
      int n = height[i];
      while (!stack.empty()) {
        if (height[stack.top()] >= n) {
          break;
        }
        size_t prev_i = stack.top();
        stack.pop();
        int prev_height = height[prev_i];
        if (!stack.empty()) {
          size_t left_i = stack.top();
          int left_height = height[left_i];

          int rec_height = std::min(n, left_height) - prev_height;
          int rec_width = i - left_i - 1;

          res += rec_height * rec_width;
        }
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
