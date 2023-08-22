// @leetup=info id=46 lang=cpp slug=permutations

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
  std::vector<std::vector<int>> res;
  std::vector<int> s;
  std::vector<int> nums;
  std::vector<bool> picked;

  std::vector<std::vector<int>> permute(std::vector<int>& nums) {
    picked = std::vector<bool>(nums.size(), false);
    this->nums = nums;
    gen();
    return res;
  }

  void gen() {
    if (s.size() == nums.size()) {
      res.push_back(s);
      return;
    }

    for (size_t j = 0; j < nums.size(); ++j) {
      if (picked[j]) {
        continue;
      }
      s.push_back(nums[j]);
      picked[j] = true;
      gen();
      s.pop_back();
      picked[j] = false;
    }
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
