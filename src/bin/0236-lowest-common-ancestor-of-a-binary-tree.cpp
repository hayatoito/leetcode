// @leetup=info id=236 lang=cpp slug=lowest-common-ancestor-of-a-binary-tree

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
  int p_val;
  int q_val;
  TreeNode* found;
  TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
    p_val = p->val;
    q_val = q->val;
    includes_p_or_q(root);
    return found;
  }

  bool includes_p_or_q(TreeNode* n) {
    if (!n) {
      return false;
    }
    int cnt = 0;
    if (n->val == p_val || n->val == q_val) {
      ++cnt;
    }
    if (includes_p_or_q(n->left)) {
      ++cnt;
    }
    if (includes_p_or_q(n->right)) {
      ++cnt;
    }
    if (cnt == 2) {
      found = n;
    }
    return cnt > 0;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
