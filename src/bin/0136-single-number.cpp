// @leetup=info id=136 lang=cpp slug=single-number

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
  int singleNumber(std::vector<int>& nums) {
    int res = 0;
    for (int n : nums) {
      res ^= n;
    }
    return res;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
