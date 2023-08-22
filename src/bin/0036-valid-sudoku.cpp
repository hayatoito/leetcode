// @leetup=info id=36 lang=cpp slug=valid-sudoku

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code
class Solution {
 public:

  bool valid(const std::vector<char>& ns) {
    std::set<int> seen;
    for (char c : ns) {
      if (c == '.') {
        continue;
      }
      if (seen.find(c) != seen.end()) {
        return false;
      }
      seen.insert(c);
    }
    return true;
  }

  bool isValidSudoku(std::vector<std::vector<char>>& board) {
    for (size_t i = 0; i <  9; ++i) {
      std::vector<char> ns0;
      std::vector<char> ns1;
      std::vector<char> ns2;
      size_t sr = (i / 3) * 3;
      size_t sc = (i % 3) * 3;
      for (size_t j = 0; j < 9; ++j) {
        ns0.push_back(board[i][j]);
        ns1.push_back(board[j][i]);
        ns2.push_back(board[sr + (j / 3)][sc + (j % 3)]);
      }
      if (!valid(ns0) || !valid(ns1) || !valid(ns2)) {
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
