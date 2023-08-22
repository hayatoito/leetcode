// @leetup=info id=268 lang=cpp slug=missing-number

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
  int missingNumber(std::vector<int>& nums) {
    int n = 0;
    for (size_t i = 0; i < nums.size(); ++i) {
      n ^= i;
      n ^= nums[i];
    }
    n ^= nums.size();
    return n;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
