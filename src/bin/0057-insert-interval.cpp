// @leetup=info id=57 lang=cpp slug=insert-interval

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
  using Interval = std::vector<int>;

  bool is_overlap(const Interval& i1, const Interval& i2) {
    return i1[0] <= i2[1] && i2[0] <= i1[1];
  }

  Interval merge_overlap(const Interval& i1, const Interval& i2) {
    return Interval{std::min(i1[0], i2[0]), std::max(i1[1], i2[1])};
  }

  std::vector<std::vector<int>> insert(std::vector<std::vector<int>>& intervals,
                                       std::vector<int>& newInterval) {
    std::vector<Interval> left;
    std::vector<Interval> right;

    for (Interval& interval : intervals) {
      if (is_overlap(interval, newInterval)) {
        newInterval = merge_overlap(interval, newInterval);
      } else if (interval[1] < newInterval[0]) {
        left.push_back(interval);
      } else {
        right.push_back(interval);
      }
    }

    left.push_back(newInterval);
    left.insert(left.end(), right.begin(), right.end());
    return left;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
