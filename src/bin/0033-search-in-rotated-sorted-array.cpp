// @leetup=info id=33 lang=cpp slug=search-in-rotated-sorted-array

#include <iostream>
#include <vector>
using namespace std;

// @leetup=code

class Solution {
 public:
  int search(vector<int>& nums, int target) {
    if (nums.size() == 1) {
      if (nums[0] == target) {
        return 0;
      }
      return -1;
    }
    int left = 0;
    int right = nums.size();
    int first = nums[0];
    int last = nums[nums.size() - 1];
    while (left < right) {
      int m = left + (right - left) / 2;
      int a = nums[m];
      if (first < last) {
        if (a < target) {
          left = m + 1;
        } else {
          right = m;
        }
      } else if (first <= target) {
        if (a <= last) {
          right = m;
        } else if (a < target) {
          left = m + 1;
        } else {
          right = m;
        }
      } else {
        if (first <= a) {
          left = m + 1;
        } else if (a < target) {
          left = m + 1;
        } else {
          right = m;
        }
      }
    }
    if (left == nums.size()) {
      return -1;
    }
    if (nums[left] == target) {
      return left;
    }
    return -1;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
