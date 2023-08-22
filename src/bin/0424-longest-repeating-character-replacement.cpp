// @leetup=info id=424 lang=cpp slug=longest-repeating-character-replacement

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  int characterReplacement(std::string s, int k) {
    // Binary Search
    int left = 0;
    int right = s.size() + 1;
    while (left < right) {
      int mid = left + (right - left) / 2;
      if (feasible(s, mid, k)) {
        left = mid + 1;
      } else {
        right = mid;
      }
    }
    return left - 1;
  }

  // Returns true if a substring of length `longest` can be made by replacing
  // `k` characters.
  bool feasible(const std::string& s, int longest, int k) {
    // A count of charaters in a sliding window:  ([i - longest + 1] .. i).
    std::map<char, int> freq;
    for (int i = 0; i < s.size(); ++i) {
      char c = s[i];
      freq[c] += 1;
      int left_out = i - longest;
      if (left_out >= 0) {
        freq[s[left_out]] -= 1;
      }
      if (freq[c] + k >= longest) {
        return true;
      }
    }
    return false;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
