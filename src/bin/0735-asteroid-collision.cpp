// @leetup=info id=735 lang=cpp slug=asteroid-collision

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <stack>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  std::vector<int> asteroidCollision(std::vector<int>& asteroids) {
    // std::stack<int> res;
    std::vector<int> res;
    for (int n : asteroids) {
      while (!res.empty()) {
        int last = res.back();
        if (last > 0 && n < 0) {
          // Both meet.
          if (last <= -n) {
            res.pop_back();
          }
          if (last >= -n) {
            n = 0;
            break;
          }
        } else {
          break;
        }
      }
      if (n != 0) {
        res.push_back(n);
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
