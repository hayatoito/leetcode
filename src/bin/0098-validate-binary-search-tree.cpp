// @leetup=info id=98 lang=cpp slug=validate-binary-search-tree

#include <algorithm>
#include <cassert>
#include <climits>
#include <cstdint>
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
  bool isValidBST(TreeNode* root) {
    return is_valid(root, INT64_MIN, INT64_MAX);
  }

  bool is_valid(TreeNode* root, int64_t low, int64_t high) {
    if (!root) {
      return true;
    }
    int val = root->val;
    return (low < val && val < high) && is_valid(root->left, low, val) &&
           is_valid(root->right, val, high);
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
