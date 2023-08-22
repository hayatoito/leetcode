// @leetup=info id=572 lang=cpp slug=subtree-of-another-tree

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
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right)
      : val(x), left(left), right(right) {}
};

// @leetup=code

class Solution {
 public:
  bool isSubtree(TreeNode* root, TreeNode* subRoot) {
    if (!subRoot) {
      return true;
    }
    if (!root) {
      return false;
    }
    return is_identical(root, subRoot) || isSubtree(root->left, subRoot) ||
           isSubtree(root->right, subRoot);
  }

  bool is_identical(TreeNode* a, TreeNode* b) {
    if (!a && !b) {
      return true;
    }
    if (a && b) {
      return a->val == b->val && is_identical(a->left, b->left) &&
             is_identical(a->right, b->right);
    }
    return false;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
