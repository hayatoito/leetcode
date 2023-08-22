// @leetup=info id=55 lang=cpp slug=jump-game

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
    bool canJump(std::vector<int>& nums) {
      size_t right = 0;
      for (size_t i = 0; i < nums.size() && i <= right; ++i) {
        right = std::max(right, i + nums[i]);
      }
      return right >= nums.size() - 1;
    }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
