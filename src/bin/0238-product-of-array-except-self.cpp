// @leetup=info id=238 lang=cpp slug=product-of-array-except-self

#include <iostream>

// @leetup=code

#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> productExceptSelf(vector<int>& nums) {
    int product = 1;
    vector<int> prefix_product{1};
    for (int& n : nums) {
      product *= n;
      prefix_product.push_back(product);
    }

    product = 1;
    vector<int> postfix_product{1};
    for (auto it = nums.rbegin(); it != nums.rend(); ++it) {
      product *= *it;
      postfix_product.push_back(product);
    }

    vector<int> res;
    for (int i = 0; i < nums.size(); ++i) {
      res.push_back(prefix_product[i] * postfix_product[nums.size() - 1 - i]);
    }
    return res;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
