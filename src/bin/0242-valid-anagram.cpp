// @leetup=info id=242 lang=cpp slug=valid-anagram

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  bool isAnagram(std::string s, std::string t) {
    std::sort(s.begin(), s.end());
    std::sort(t.begin(), t.end());
    return s == t;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
