// @leetup=info id=78 lang=cpp slug=subsets

#include <algorithm>
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
  std::vector<int>* nums;

  std::vector<std::vector<int>> subsets(std::vector<int>& nums) {
    this->nums = &nums;
    gen(0);
    return res;
  }

  void gen(size_t i) {
    if (i == nums->size()) {
      res.push_back(s);
      return;
    }

    // Pick ith element.
    s.push_back((*nums)[i]);
    gen(i + 1);
    s.pop_back();

    // Skip ith element.
    gen(i + 1);
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
