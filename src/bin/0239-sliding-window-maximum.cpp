// @leetup=info id=239 lang=cpp slug=sliding-window-maximum

#include <iostream>

// @leetup=code

#include <deque>
#include <vector>

class Solution {
 public:
  std::vector<int> maxSlidingWindow(std::vector<int>& nums, int k) {
    std::deque<int> q;
    std::vector<int> res;
    for (int i = 0; i < nums.size(); ++i) {
      int n = nums[i];
      while (!q.empty() && nums[q.back()] <= n) {
        q.pop_back();
      }
      q.push_back(i);
      if (q.front() < i - (k - 1)) {
        q.pop_front();
      }
      if (i >= k - 1) {
        res.push_back(nums[q.front()]);
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
