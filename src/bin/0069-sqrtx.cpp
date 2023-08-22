// @leetup=info id=69 lang=cpp slug=sqrtx

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
  int mySqrt(int x) {
    if (x == 0) {
      return 0;
    }
    uint64_t x1 = x;
    uint64_t low = 0;
    uint64_t high = x1 + 1;

    while (low < high) {
      uint64_t m = low + (high - low) / 2;
      if (m * m <= static_cast<uint64_t>(x1)) {
        low = m + 1;
      } else {
        high = m;
      }
    }
    return static_cast<int>(low - 1);
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
