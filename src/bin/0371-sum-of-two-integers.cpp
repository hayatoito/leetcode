// @leetup=info id=371 lang=cpp slug=sum-of-two-integers

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
  int getSum(int a, int b) {
    return static_cast<int>(
        getSum1(static_cast<uint32_t>(a), static_cast<uint32_t>(b)));
  }

  uint32_t getSum1(uint32_t a, uint32_t b) {
    while (b != 0) {
      uint32_t tmp_a = a;
      a = a ^ b;
      b = (tmp_a & b) << 1;
    }
    return a;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
