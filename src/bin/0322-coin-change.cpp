// @leetup=info id=322 lang=cpp slug=coin-change

#include <algorithm>
#include <cassert>
#include <climits>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  int coinChange(std::vector<int>& coins, int amount) {
    std::vector<size_t> dp(amount + 1, INT_MAX);
    dp[0] = 0;
    for (int i = 1; i <= amount; ++i) {
      for (int coin : coins) {
        if (i - coin >= 0) {
          dp[i] = std::min(dp[i], dp[i - coin] + 1);
        }
      }
    }
    return dp[amount] == INT_MAX ? -1 : dp[amount];
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
