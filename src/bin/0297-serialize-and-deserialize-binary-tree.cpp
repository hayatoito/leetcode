// @leetup=info id=297 lang=cpp slug=serialize-and-deserialize-binary-tree

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

// @leetup=code

#include <sstream>
#include <iterator>

class Codec {
 public:
  // Encodes a tree to a single string.
  std::string serialize(TreeNode* root) {
    std::vector<std::string> collect;
    serialize_dfs(root, collect);

    // Join collect with " ".
    std::string ss;
    for (auto it = collect.begin(); it != collect.end(); it++) {
        if (it != collect.begin()) {
            ss += " ";
        }
        ss += *it;
    }
    return ss;
  }

  void serialize_dfs(TreeNode* n, std::vector<std::string>& collect) {
    if (!n) {
      collect.push_back("null");
      return;
    }
    collect.push_back(std::to_string(n->val));
    serialize_dfs(n->left, collect);
    serialize_dfs(n->right, collect);
  }

  // Decodes your encoded data to tree.
  TreeNode* deserialize(std::string data) {
    std::istringstream in(data);
    return deserialize_dfs(in);
  }

  TreeNode* deserialize_dfs(std::istringstream& in) {
    std::string val;
    in >> val;
    if (val == "null") {
      return nullptr;
    }
    TreeNode* node = new TreeNode(std::stoi(val));
    node->left = deserialize_dfs(in);
    node->right = deserialize_dfs(in);
    return node;
  }
};

// Your Codec object will be instantiated and called as such:
// Codec ser, deser;
// TreeNode* ans = deser.deserialize(ser.serialize(root));
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
