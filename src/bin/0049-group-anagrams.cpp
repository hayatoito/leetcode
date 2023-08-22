// @leetup=info id=49 lang=cpp slug=group-anagrams

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  std::vector<std::vector<std::string>> groupAnagrams(
      std::vector<std::string>& strs) {
    std::map<std::string, std::vector<std::string>> groups;
    for (std::string& s : strs) {
      std::string sorted = s;
      std::sort(sorted.begin(), sorted.end());
      groups[sorted].push_back(s);
    }

    std::vector<std::vector<std::string>> res;
    for (auto& iter : groups) {
      res.push_back(iter.second);
    }
    return res;
  };
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
