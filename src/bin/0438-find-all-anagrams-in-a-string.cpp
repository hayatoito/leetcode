// @leetup=info id=438 lang=cpp slug=find-all-anagrams-in-a-string

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  std::vector<int> findAnagrams(std::string s, std::string p) {
    std::map<char, int> p_counter;
    for (char c : p) {
      p_counter[c] += 1;
    }

    std::vector<int> res;
    std::map<char, int> s_counter;
    for (int i = 0; i < s.size(); ++i) {
      char c = s[i];
      s_counter[c] += 1;
      int start_index = i - p.size() + 1;
      if (start_index - 1 >= 0) {
        s_counter[s[start_index - 1]] -= 1;
      }
      if (p_counter == s_counter) {
        res.push_back(start_index);
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
