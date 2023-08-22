// @leetup=info id=84 lang=cpp slug=largest-rectangle-in-histogram

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
  int largestRectangleArea(std::vector<int>& heights) {
    // Add sentinels.
    heights.insert(heights.begin(), 0);
    heights.push_back(0);

    std::stack<size_t> stack;
    int res = 0;
    for (size_t i = 0; i < heights.size(); ++i) {
      int n = heights[i];
      while (!stack.empty() && heights[stack.top()] > n) {
        size_t prev_i = stack.top();
        stack.pop();
        int prev_height = heights[prev_i];
        if (!stack.empty()) {
          size_t left_i = stack.top();

          int rect_width = i - left_i - 1;
          int area = rect_width * prev_height;

          res = std::max(res, area);
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
