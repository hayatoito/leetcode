// @leetup=info id=11 lang=cpp slug=container-with-most-water

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>
using std::vector;

// @leetup=code
class Solution {
 public:
  int maxArea(vector<int>& height) {
    int res = 0;
    int left = 0;
    int right = height.size() - 1;
    while (left < right) {
      res =
          std::max(res, std::min(height[left], height[right]) * (right - left));
      if (height[left] < height[right]) {
        left += 1;
      } else {
        right -= 1;
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
