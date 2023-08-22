// @leetup=info id=230 lang=cpp slug=kth-smallest-element-in-a-bst

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
  int kthSmallest(TreeNode* root, int k) {
    std::vector<int> collect;
    visit(root, collect);
    return collect[static_cast<size_t>(k) - 1];
  }

  void visit(TreeNode* root, std::vector<int>& collect) {
    if (!root) {
      return;
    }
    visit(root->left, collect);
    collect.push_back(root->val);
    visit(root->right, collect);
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
