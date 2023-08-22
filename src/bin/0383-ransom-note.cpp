// @leetup=info id=383 lang=cpp slug=ransom-note

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  bool canConstruct(std::string ransomNote, std::string magazine) {
    std::map<char, int> a;
    for (char c : ransomNote) {
      a[c] += 1;
    }
    std::map<char, int> b;
    for (char c : magazine) {
      b[c] += 1;
    }
    for (auto it : a) {
      if (b[it.first] < it.second) {
        return false;
      }
    }
    return true;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
