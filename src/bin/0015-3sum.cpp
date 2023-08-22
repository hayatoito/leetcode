// @leetup=info id=15 lang=cpp slug=3sum

#include <algorithm>
#include <iostream>
#include <set>
#include <tuple>
#include <vector>

using std::vector;

// using namespace std;

// @leetup=code
class Solution {
 public:
  vector<vector<int>> threeSum(vector<int>& nums) {
    std::sort(nums.begin(), nums.end());
    std::set<vector<int>> s;
    for (int i = 0; i < nums.size(); ++i) {
      int j = i + 1;
      int k = nums.size() - 1;
      while (j < k) {
        int sum = nums[i] + nums[j] + nums[k];
        if (sum == 0) {
          s.insert({nums[i], nums[j], nums[k]});
          j += 1;
          k -= 1;
        } else if (sum < 0) {
          j += 1;
        } else {
          k -= 1;
        }
      }
    }

    vector<vector<int>> res(s.begin(), s.end());
    return res;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
