// @leetup=info id=104 lang=cpp slug=maximum-depth-of-binary-tree

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
  int maxDepth(TreeNode* root) {
    if (!root) {
      return 0;
    }
    return 1 + std::max(maxDepth(root->left), maxDepth(root->right));
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
