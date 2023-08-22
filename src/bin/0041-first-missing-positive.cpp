// @leetup=info id=41 lang=cpp slug=first-missing-positive

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
  int firstMissingPositive(std::vector<int>& nums) {
    for (size_t i = 0; i < nums.size(); ++i) {
      if (nums[i] < 0) {
        nums[i] = 0;
      }
    }

    // 0: nothing
    // +3:
    // -3: [this index of element is marked]

    for (size_t i = 0; i < nums.size(); ++i) {
      int val = nums[i];
      if (val == 0 || val == -1) {
        continue;
      }
      if (val < 0) {
        // Restore the origival value
        // origi_value = 3
        // val = -4 = (-3 - 1)
        val = -val - 1;
      }
      assert(val > 0);
      if (static_cast<size_t>(val) <= nums.size()) {
        int index = val - 1;
        // Mark the index using minus value.
        // minus 1 to distinguich from zero.
        if (nums[index] >= 0) {
          nums[index] = -nums[index] - 1;
        }
      }
    }
    for (size_t i = 1; i <= nums.size(); ++i) {
      if (nums[i - 1] >= 0) {
        return i;
      }
    }
    return nums.size() + 1;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
