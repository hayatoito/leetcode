// @leetup=info id=54 lang=cpp slug=spiral-matrix

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  std::vector<int> spiralOrder(std::vector<std::vector<int>>& matrix) {
    std::vector<int> res;

    size_t low_r = 0;
    size_t high_r = matrix.size();

    size_t low_c = 0;
    size_t high_c = matrix[0].size();

    size_t r = 0;
    size_t c = 0;

    size_t direction = 0;
    std::vector<std::tuple<int, int>> d{{0, 1}, {1, 0}, {0, -1}, {-1, 0}};

    while (low_r <= r && r < high_r && low_c <= c && c < high_c) {
      res.push_back(matrix[r][c]);
      size_t nr = r + std::get<0>(d[direction]);
      size_t nc = c + std::get<1>(d[direction]);
      if (low_r <= nr && nr < high_r && low_c <= nc && nc < high_c) {
        r = nr;
        c = nc;
      } else {
        if (direction == 0) {
          low_r += 1;
        } else if (direction == 1) {
          high_c -= 1;
        } else if (direction == 2) {
          high_r -= 1;
        } else {
          low_c += 1;
        }
        direction = (direction + 1) % 4;
        r += std::get<0>(d[direction]);
        c += std::get<1>(d[direction]);
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
