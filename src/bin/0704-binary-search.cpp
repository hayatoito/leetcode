// @leetup=info id=704 lang=cpp slug=binary-search

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  int search(std::vector<int>& nums, int target) {
    auto it = std::lower_bound(nums.begin(), nums.end(), target);
    if (it == nums.end() || *it != target) {
      return -1;
    }
    return it - nums.begin();
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
