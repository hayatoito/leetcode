// @leetup=info id=53 lang=cpp slug=maximum-subarray

#include <iostream>

// @leetup=code

#include <vector>
using namespace std;

class Solution {
 public:
  int maxSubArray(vector<int>& nums) {
    int best = nums[0];
    int pre = nums[0];
    for (int i = 1; i < nums.size(); ++i) {
      pre = max(nums[i], pre + nums[i]);
      best = max(best, pre);
    }
    return best;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
