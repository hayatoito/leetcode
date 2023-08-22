// @leetup=info id=1 lang=cpp slug=two-sum

#include <algorithm>
#include <cstdint>
#include <iostream>
#include <unordered_map>
#include <vector>
using namespace std;

// @leetup=code

#include <assert.h>

class Solution {
 public:
  vector<int> twoSum(vector<int>& nums, int target) {
    unordered_map<int, int> visited;
    for (size_t i = 0; i < nums.size(); ++i) {
      int m = target - nums[i];
      auto found = visited.find(m);
      if (found != visited.end()) {
        return {static_cast<int>(i), found->second};
      }
      visited[nums[i]] = i;
    }
    assert(false);
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
