// @leetup=info id=836 lang=cpp slug=rectangle-overlap

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
  bool isRectangleOverlap(std::vector<int>& rec1, std::vector<int>& rec2) {
    return isSegmentOverlap({rec1[0], rec1[2]}, {rec2[0], rec2[2]}) &&
           isSegmentOverlap({rec1[1], rec1[3]}, {rec2[1], rec2[3]});
  }

 private:
  bool isSegmentOverlap(std::tuple<int, int> seg1, std::tuple<int, int> seg2) {
    return std::get<0>(seg1) < std::get<1>(seg2) &&
           std::get<0>(seg2) < std::get<1>(seg1);
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
