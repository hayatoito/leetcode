// @leetup=info id=199 lang=cpp slug=binary-tree-right-side-view

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
  std::vector<int> rightSideView(TreeNode* root) {
    std::vector<int> res;
    if (!root) {
      return res;
    }
    std::queue<TreeNode*> q;
    q.push(root);
    while (!q.empty()) {
      bool poped = false;
      std::queue<TreeNode*> next_q;
      while (!q.empty()) {
        TreeNode* n = q.front();
        q.pop();

        if (!poped) {
          res.push_back(n->val);
        }
        poped = true;

        if (n->right) {
          next_q.push(n->right);
        }
        if (n->left) {
          next_q.push(n->left);
        }
      }
      std::swap(q, next_q);
    }
    return res;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
