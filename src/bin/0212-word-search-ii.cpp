// @leetup=info id=212 lang=cpp slug=word-search-ii

#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

struct Node {
  std::array<std::unique_ptr<Node>, 26> children{};
  std::optional<std::string> word;
};

class Solution {
 public:
  std::vector<std::string> findWords(std::vector<std::vector<char>>& board,
                                     std::vector<std::string>& words) {
    this->board_ = board;
    Node root;
    for (auto& word : words) {
      Node* node = &root;
      for (char c : word) {
        if (!node->children[c - 'a']) {
          node->children[c - 'a'] = std::make_unique<Node>();
        }
        node = node->children[c - 'a'].get();
      }
      node->word = std::move(word);
    }

    for (size_t r = 0; r < board.size(); ++r) {
      for (size_t c = 0; c < board[0].size(); ++c) {
        dfs(r, c, &root);
      }
    }

    return res_;
  }

 private:
  void dfs(size_t r, size_t c, Node* node) {
    if (!(r < board_.size() && c < board_[0].size())) {
      return;
    }

    char x = board_[r][c];
    if (x == '*') {
      return;
    }

    Node* n = node->children[x - 'a'].get();
    if (!n) {
      return;
    }

    if (n->word) {
      res_.push_back(*n->word);
      n->word = std::nullopt;
    }

    board_[r][c] = '*';

    dfs(r, c + 1, n);
    dfs(r, c - 1, n);
    dfs(r + 1, c, n);
    dfs(r - 1, c, n);

    board_[r][c] = x;
  }

  std::vector<std::string> res_;
  std::vector<std::vector<char>> board_;
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
