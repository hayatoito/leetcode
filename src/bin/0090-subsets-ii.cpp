// @leetup=info id=90 lang=cpp slug=subsets-ii

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  std::set<std::vector<int>> res;
  std::vector<int> s;
  std::vector<int> nums;

  std::vector<std::vector<int>> subsetsWithDup(std::vector<int>& nums) {
    std::sort(nums.begin(), nums.end());
    this->nums = nums;
    dfs(0);
    std::vector<std::vector<int>> r(res.begin(), res.end());
    return r;
  }

  void dfs(size_t i) {
    if (i == nums.size()) {
      res.insert(s);
      return;
    }

    s.push_back(nums[i]);
    dfs(i + 1);
    s.pop_back();

    dfs(i + 1);
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
