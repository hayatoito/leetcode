// @leetup=info id=191 lang=cpp slug=number-of-1-bits

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
  int hammingWeight(uint32_t n) {
    int res = 0;
    while (n != 0) {
      ++res;
      n = n & (n - 1);
    }
    return res;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
