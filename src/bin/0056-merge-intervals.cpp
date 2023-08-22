// @leetup=info id=56 lang=cpp slug=merge-intervals

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
  std::vector<std::vector<int>> merge(
      std::vector<std::vector<int>>& intervals) {
    std::vector<std::vector<int>> res;
    std::sort(intervals.begin(), intervals.end());

    for (std::vector<int>& interval : intervals) {
      if (!res.empty() && res.back()[1] >= interval[0]) {
        res.back()[1] = std::max(res.back()[1], interval[1]);
      } else {
        res.push_back(interval);
      }
    }
    return res;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
