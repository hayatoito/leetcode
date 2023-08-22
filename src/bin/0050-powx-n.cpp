// @leetup=info id=50 lang=cpp slug=powx-n

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
  double myPow(double x, int n) {
    if (n == 0) {
      return 1.0;
    }
    if (n > 0) {
      return pow(x, n);
    }
    return 1.0 / pow(x, -static_cast<int64_t>(n));
  }

 private:
  double pow(double x, uint64_t n) {
    double res = 1.0;
    double p = x;
    while (n != 0) {
      if (n & 1) {
        res *= p;
      }
      n >>= 1;
      p *= p;
    }
    return res;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
