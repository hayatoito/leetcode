// @leetup=info id=102 lang=cpp slug=binary-tree-level-order-traversal

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <tuple>
#include <vector>

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right)
      : val(x), left(left), right(right) {}
};

// @leetup=code

class Solution {
 public:
  std::vector<std::vector<int>> levelOrder(TreeNode* root) {
    std::vector<std::vector<int>> res;
    if (!root) {
      return res;
    }

    std::vector<TreeNode*> q;
    q.push_back(root);

    while (!q.empty()) {
      std::vector<int> level;
      std::vector<TreeNode*> next_q;
      for (TreeNode* n : q) {
        level.push_back(n->val);
        if (n->left) {
          next_q.push_back(n->left);
        }
        if (n->right) {
          next_q.push_back(n->right);
        }
      }
      res.push_back(level);
      std::swap(q, next_q);
    }

    return res;
  }
};
// @leetup=code

i nt main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
