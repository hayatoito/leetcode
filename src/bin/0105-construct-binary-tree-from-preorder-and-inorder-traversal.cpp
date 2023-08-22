// @leetup=info id=105 lang=cpp
// slug=construct-binary-tree-from-preorder-and-inorder-traversal

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
  using Iter = std::vector<int>::const_iterator;

  TreeNode* buildTree(std::vector<int>& preorder, std::vector<int>& inorder) {
    preorder_it = preorder.cbegin();
    return build(inorder.cbegin(), inorder.cend());
  }

  TreeNode* build(Iter inorder_begin, Iter inorder_end) {
    if (inorder_begin == inorder_end) {
      return nullptr;
    }
    int root = *preorder_it;
    ++preorder_it;
    Iter root_it = std::find(inorder_begin, inorder_end, root);
    return new TreeNode(root, build(inorder_begin, root_it),
                        build(root_it + 1, inorder_end));
  }

 private:
  Iter preorder_it;
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
