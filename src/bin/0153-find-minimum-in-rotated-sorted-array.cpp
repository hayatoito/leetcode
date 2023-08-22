// @leetup=info id=153 lang=cpp slug=find-minimum-in-rotated-sorted-array

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  int findMin(std::vector<int>& nums) {
    if (nums.size() == 1 || nums.front() < nums.back()) {
      return nums.front();
    }
    return *std::lower_bound(nums.begin(), nums.end(), nums.front(),
                             [](int n, int front) { return front <= n; });
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
