// @leetup=info id=338 lang=cpp slug=counting-bits

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
  std::vector<int> countBits(int n) {
    std::vector<int> res;
    for (int i = 0; i <= n; ++i) {
      if (i == 0) {
        res.push_back(0);
      } else {
        res.push_back(res[i / 2] + (i % 2));
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
