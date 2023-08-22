// @leetup=info id=211 lang=cpp slug=design-add-and-search-words-data-structure

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

struct Node {
  bool is_word = false;
  std::map<char, Node> children;
};

class WordDictionary {
 public:
  WordDictionary() = default;

  void addWord(std::string word) {
    Node* node = &root_;
    for (char c : word) {
      node = &node->children[c];
    }
    node->is_word = true;
  }

  bool search(std::string word) {
    return dfs(word.begin(), word.end(), root_);
  }

 private:
  bool dfs(std::string::iterator it, std::string::iterator end, Node& node) {
    if (it == end) {
      return node.is_word;
    }
    if (*it == '.') {
      for (auto& [_, child] : node.children) {
        if (dfs(std::next(it), end, child)) {
          return true;
        }
      }
      return false;
    }
    auto child_it = node.children.find(*it);
    if (child_it == node.children.end()) {
      return false;
    }
    return dfs(std::next(it), end, child_it->second);
  }

  Node root_;
};

/**
 * Your WordDictionary object will be instantiated and called as such:
 * WordDictionary* obj = new WordDictionary();
 * obj->addWord(word);
 * bool param_2 = obj->search(word);
 */
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
