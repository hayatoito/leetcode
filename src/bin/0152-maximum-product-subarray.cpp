// @leetup=info id=152 lang=cpp slug=maximum-product-subarray

#include <climits>
#include <iostream>

// @leetup=code

#include <vector>
using namespace std;

class Solution {
 public:
  int maxProduct(vector<int>& nums) {
    int positive_best = 1;
    int negative_best = 1;
    int res = INT_MIN;
    for (int& n : nums) {
      if (n == 0) {
        positive_best = 1;
        negative_best = 1;
        res = max(res, 0);
      } else {
        int p_temp = max(max(positive_best * n, negative_best * n), n);
        negative_best = min(min(positive_best * n, negative_best * n), n);
        positive_best = p_temp;
        res = max(res, positive_best);
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
