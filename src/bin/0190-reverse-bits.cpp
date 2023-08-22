// @leetup=info id=190 lang=cpp slug=reverse-bits

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
  uint32_t reverseBits(uint32_t n) {
    uint32_t res = 0;
    for (size_t i = 0; i < 32; ++i) {
      res = (res << 1) | (n & 1);
      n = n >> 1;
    }
    return res;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
