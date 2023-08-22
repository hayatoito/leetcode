// @leetup=info id=217 lang=cpp slug=contains-duplicate

#include <iostream>

// @leetup=code

#include <unordered_set>
#include <vector>
using namespace std;

class Solution {
 public:
  bool containsDuplicate(vector<int>& nums) {
    unordered_set<int> set(nums.begin(), nums.end());
    return set.size() != nums.size();
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
