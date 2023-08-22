// @leetup=info id=100 lang=cpp slug=same-tree

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
  bool isSameTree(TreeNode* p, TreeNode* q) {
    if (!p && !q) {
      return true;
    }
    if (p && !q) {
      return false;
    }
    if (!p && q) {
      return false;
    }
    return p->val == q->val && isSameTree(p->left, q->left) &&
           isSameTree(p->right, q->right);
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
