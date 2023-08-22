// @leetup=info id=124 lang=cpp slug=binary-tree-maximum-path-sum

#include <algorithm>
#include <cassert>
#include <climits>
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
  int res = INT_MIN;

  int maxPathSum(TreeNode* root) {
    maxPath(root);
    return res;
  }

  int maxPath(TreeNode* n) {
    if (!n) {
      return 0;
    }
    int left_path = maxPath(n->left);
    int right_path = maxPath(n->right);

    // maxPath can be negative.
    int max_path =
        std::max(n->val, std::max(n->val + left_path, n->val + right_path));

    res = std::max(res, max_path);
    res = std::max(res, n->val + left_path + right_path);

    return max_path;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
