// @leetup=info id=223 lang=cpp slug=rectangle-area

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
  int computeArea(int ax1,
                  int ay1,
                  int ax2,
                  int ay2,
                  int bx1,
                  int by1,
                  int bx2,
                  int by2) {
    int area_a = (ax2 - ax1) * (ay2 - ay1);
    int area_b = (bx2 - bx1) * (by2 - by1);
    int area_overlap = intervalIntersection({ax1, ax2}, {bx1, bx2}) *
                       intervalIntersection({ay1, ay2}, {by1, by2});
    return area_a + area_b - area_overlap;
  }

 private:
  int intervalIntersection(std::tuple<int, int> seg1,
                           std::tuple<int, int> seg2) {
    int min = std::max(std::get<0>(seg1), std::get<0>(seg2));
    int max = std::min(std::get<1>(seg1), std::get<1>(seg2));
    return std::max(max - min, 0);
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
