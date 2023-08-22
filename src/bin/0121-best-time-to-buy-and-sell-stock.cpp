// @leetup=info id=121 lang=cpp slug=best-time-to-buy-and-sell-stock

// @leetup=code

#include <iostream>
// [2023-04-25 Tue] leetcode doesn't support c++20.
// #include <span>
#include <algorithm>
#include <vector>
using namespace std;

// class Solution {
//  public:
//   int maxProfit(vector<int>& prices) {
//     int max_profit = 0;
//     int min_price = prices[0];
//     // for (auto&& price : std::span{prices}.subspan(1)) {
//     // vector<int> s = {prices.begin() + 1, prices.end()};
//     // for (const int& price : s) {
//     for (const int& price : vector{prices.begin() + 1, prices.end()}) {
//       max_profit = max(max_profit, price - min_price);
//       min_price = min(min_price, price);
//     }
//     return max_profit;
//   }
// };

class Solution {
 public:
  int maxProfit(vector<int>& prices) {
    int max_profit = 0;
    int min_price = prices[0];
    // for (const int& price : vector{prices.cbegin() + 1, prices.cend()}) {
    for (const int& price : vector(prices.cbegin() + 1, prices.cend())) {
      max_profit = max(max_profit, price - min_price);
      min_price = min(min_price, price);
    }
    return max_profit;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
