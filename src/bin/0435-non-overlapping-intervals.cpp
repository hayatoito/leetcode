// @leetup=info id=435 lang=cpp slug=non-overlapping-intervals

#include <algorithm>
#include <cassert>
#include <climits>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  int eraseOverlapIntervals(std::vector<std::vector<int>>& intervals) {
    std::sort(intervals.begin(), intervals.end());
    int res = 0;
    // Sentinel.
    int end = INT_MAX;
    for (std::vector<int>& interval : intervals) {
      if (interval[0] < end) {
        // Overlapping.
        res += 1;
        end = std::min(end, interval[1]);
      } else {
        end = interval[1];
      }
    }
    return res - 1;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
