// @leetup=info id=226 lang=cpp slug=invert-binary-tree

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
  TreeNode* invertTree(TreeNode* root) {
    if (!root) {
      return nullptr;
    }
    TreeNode* tmp = invertTree(root->left);
    root->left = invertTree(root->right);
    root->right = tmp;
    return root;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
