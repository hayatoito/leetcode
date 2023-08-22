// @leetup=info id=347 lang=cpp slug=top-k-frequent-elements

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  std::vector<int> topKFrequent(std::vector<int>& nums, int k) {
    std::map<int, size_t> count;
    for (int n : nums) {
      ++count[n];
    }
    std::priority_queue<std::tuple<size_t, int>> q;
    for (auto [key, cnt] : count) {
      q.push({-cnt, key});
      if (q.size() > static_cast<size_t>(k)) {
        q.pop();
      }
    }
    std::vector<int> res;
    while (!q.empty()) {
      auto [_cnt, k] = q.top();
      q.pop();
      res.push_back(k);
    }
    return res;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
