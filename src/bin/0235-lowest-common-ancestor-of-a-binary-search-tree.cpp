// @leetup=info id=235 lang=cpp
// slug=lowest-common-ancestor-of-a-binary-search-tree

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

class Solution {
 public:
  TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
    if (p->val > q->val) {
      std::swap(p, q);
    }
    assert(p->val < q->val);
    while (true) {
      if (root == p || root == q) {
        return root;
      }
      if (p->val < root->val && root->val < q->val) {
        return root;
      }
      if (q->val < root->val) {
        root = root->left;
      } else {
        assert(root->val < p->val);
        root = root->right;
      }
    }
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
