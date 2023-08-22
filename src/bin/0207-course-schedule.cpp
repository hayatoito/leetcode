// @leetup=info id=207 lang=cpp slug=course-schedule

#include <algorithm>
#include <cassert>
#include <cstddef>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  bool canFinish(int numCourses, std::vector<std::vector<int>>& prerequisites) {
    size_t N = static_cast<size_t>(numCourses);
    std::vector<size_t> in_cnt(numCourses);
    std::vector<std::vector<size_t>> out_nodes(numCourses);
    for (auto& prereq : prerequisites) {
      size_t a = prereq[0];
      size_t b = prereq[1];
      ++in_cnt[b];
      out_nodes[a].push_back(b);
    }

    std::queue<size_t> q;
    for (size_t i = 0; i < N; ++i) {
      if (in_cnt[i] == 0) {
        q.push(i);
      }
    }

    size_t took = 0;

    while (!q.empty()) {
      ++took;
      size_t n = q.front();
      q.pop();
      for (size_t out : out_nodes[n]) {
        --in_cnt[out];
        if (in_cnt[out] == 0) {
          q.push(out);
        }
      }
    }

    return took == N;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
