// @leetup=info id=73 lang=cpp slug=set-matrix-zeroes

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  void setZeroes(std::vector<std::vector<int>>& matrix) {
    std::set<size_t> zero_r;
    std::set<size_t> zero_c;

    for (size_t r = 0; r < matrix.size(); ++r) {
      for (size_t c = 0; c < matrix[0].size(); ++c) {
        if (matrix[r][c] == 0) {
          zero_r.insert(r);
          zero_c.insert(c);
        }
      }
    }

    for (size_t r = 0; r < matrix.size(); ++r) {
      for (size_t c = 0; c < matrix[0].size(); ++c) {
        if (zero_r.find(r) != zero_r.end() || zero_c.find(c) != zero_c.end()) {
          matrix[r][c] = 0;
        }
      }
    }
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
