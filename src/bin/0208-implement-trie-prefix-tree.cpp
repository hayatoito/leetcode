// @leetup=info id=208 lang=cpp slug=implement-trie-prefix-tree

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

class Trie {
 public:
  Trie() = default;

  void insert(std::string word) {
    Node* node = &root_;
    for (char c : word) {
      node = &node->children[c];
    }
    node->is_word = true;
  }

  bool search(std::string word) {
    Node* n = searchNode(word);
    return n && n->is_word;
  }

  bool startsWith(std::string prefix) {
    return searchNode(prefix);
  }

 private:
  Node* searchNode(const std::string s) {
    Node* node = &root_;
    for (char c : s) {
      auto it = node->children.find(c);
      if (it == node->children.end()) {
        return nullptr;
      }
      node = &it->second;
    }
    return node;
  }

  Node root_;
};

/**
 * Your Trie object will be instantiated and called as such:
 * Trie* obj = new Trie();
 * obj->insert(word);
 * bool param_2 = obj->search(word);
 * bool param_3 = obj->startsWith(prefix);
 */
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
