// @leetup=info id=77 lang=cpp slug=combinations

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  std::vector<std::vector<int>> res;
  std::vector<int> s;
  int n;

  std::vector<std::vector<int>> combine(int n, int k) {
    this->n = n;
    dfs(0, k);
    return res;
  }

  void dfs(int pick, int remain) {
    if (remain == 0) {
      res.push_back(s);
      return;
    }
    if (pick == n) {
      return;
    }

    // Pick
    s.push_back(pick + 1);
    dfs(pick + 1, remain - 1);
    s.pop_back();

    // Don't pick
    dfs(pick + 1, remain);
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
