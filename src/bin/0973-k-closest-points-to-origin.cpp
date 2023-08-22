// @leetup=info id=973 lang=cpp slug=k-closest-points-to-origin

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code
class Reverse {
 public:
  bool operator()(const std::vector<int>& a, const std::vector<int>& b) {
    return a[0] * a[0] + a[1] * a[1] < b[0] * b[0] + b[1] * b[1];
  }
};

class Solution {
 public:
  std::vector<std::vector<int>> kClosest(std::vector<std::vector<int>>& points,
                                         int k) {
    std::priority_queue<std::vector<int>, std::vector<std::vector<int>>,
                        Reverse>
        q;
    for (auto& point : points) {
      q.push(point);
      if (q.size() > static_cast<size_t>(k)) {
        q.pop();
      }
    }

    std::vector<std::vector<int>> res;
    while (!q.empty()) {
      res.emplace_back(std::move(q.top()));
      q.pop();
    }
    return res;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
