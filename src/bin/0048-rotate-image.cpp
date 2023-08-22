// @leetup=info id=48 lang=cpp slug=rotate-image

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  void rotate(std::vector<std::vector<int>>& matrix) {
    size_t left = 0;
    size_t right = matrix.size() - 1;

    while (left < right) {
      for (size_t i = 0; i < (right - left) ; ++ i )  {
        // Save the top_left.
        int top_left = matrix[left][left + i];
        // Move bottom_left into top_left
        matrix[left][left + i] = matrix[right - i][left];
        // Move bottom_right into bottom_left
        matrix[right - i][left] = matrix[right][right - i];
        // Move top_right into bottom_right
        matrix[right][right - i] = matrix[left + i][right];
        // Move top_left into top_right
        matrix[left + i][right] = top_left;
      }

      left += 1;
      right -= 1;
    }
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
