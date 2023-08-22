// @leetup=info id=4 lang=cpp slug=median-of-two-sorted-arrays

#include <algorithm>
#include <cassert>
#include <climits>
#include <cstdint>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  double findMedianSortedArrays(std::vector<int>& nums1,
                                std::vector<int>& nums2) {
    if (nums1.size() > nums2.size()) {
      return findMedianSortedArrays(nums2, nums1);
    }

    size_t total_size = nums1.size() + nums2.size();
    size_t half_size = total_size / 2;

    size_t left = 0;
    size_t right = nums1.size();

    // This should be left <= right, instead of left < right because
    //  we need to run this loop for left==right at last.
    while (left <= right) {
      size_t mid1 = left + (right - left) / 2;
      size_t mid2 = half_size - mid1;

      int left1 = (mid1 >= 1) ? nums1[mid1 - 1] : INT_MIN;
      int right1 = (mid1 < nums1.size()) ? nums1[mid1] : INT_MAX;

      int left2 = (mid2 >= 1) ? nums2[mid2 - 1] : INT_MIN;
      int right2 = (mid2 < nums2.size()) ? nums2[mid2] : INT_MAX;

      if (left1 <= right2 && left2 <= right1) {
        if (total_size % 2 == 1) {
          return std::min(right1, right2);
        }
        return (std::max(left1, left2) + std::min(right1, right2)) / 2.0;
      }
      if (left1 < right2) {
        left = mid1 + 1;
      } else {
        right = mid1;
      }
    }
    assert(false);
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
