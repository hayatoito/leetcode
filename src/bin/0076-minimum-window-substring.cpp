// @leetup=info id=76 lang=cpp slug=minimum-window-substring

#include <algorithm>
#include <cassert>
#include <climits>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  std::string minWindow(std::string s, std::string t) {
    std::map<char, int> t_counter;
    for (char c : t) {
      t_counter[c] += 1;
    }
    const int t_goal = t_counter.size();

    // Sliding window

    // When goal reaches t_counter.size(), sliding window includes t.
    int goal = 0;

    std::map<char, int> s_counter;
    int left = 0;

    int min_window_size = INT_MAX;
    int min_window_left = -1;

    for (int right = 0; right < s.size(); ++right) {
      // Update the sliding window's state in new `right`.
      char c = s[right];
      s_counter[c] += 1;

      if (s_counter[c] == t_counter[c]) {
        goal += 1;
      }

      // Shrink the sliding window while goal is satisfied.
      while (left <= right && goal == t_goal) {
        // Recorrd the result.
        int window_size = right - left + 1;
        if (window_size < min_window_size) {
          min_window_size = window_size;
          min_window_left = left;
        }

        // Update the state of the sliding window, assuming left` will be out of
        // the window.
        char left_c = s[left];
        if ((t_counter.find(left_c) != t_counter.end()) &&
            s_counter[left_c] == t_counter[left_c]) {
          goal -= 1;
          assert(goal >= 0);
        }
        s_counter[left_c] -= 1;

        left += 1;
      }
    }
    if (min_window_left == -1) {
      return "";
    }
    return s.substr(min_window_left, min_window_size);
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
