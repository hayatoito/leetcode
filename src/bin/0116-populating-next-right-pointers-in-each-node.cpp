// @leetup=info id=116 lang=cpp slug=populating-next-right-pointers-in-each-node

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// Definition for a Node.
class Node {
 public:
  int val;
  Node* left;
  Node* right;
  Node* next;

  Node() : val(0), left(NULL), right(NULL), next(NULL) {}

  Node(int _val) : val(_val), left(NULL), right(NULL), next(NULL) {}

  Node(int _val, Node* _left, Node* _right, Node* _next)
      : val(_val), left(_left), right(_right), next(_next) {}
};

// @leetup=code

class Solution {
 public:
  // spece: O(1)
  Node* connect(Node* root) {
    Node* leftmost = root;
    while (leftmost) {
      Node* cur = leftmost;
      while (cur && cur->left) {
        cur->left->next = cur->right;
        if (cur->next) {
          cur->right->next = cur->next->left;
        }
        cur = cur->next;
      }
      leftmost = leftmost->left;
    }
    return root;
  }

  // // spece: O(logN)
  // Node* connect(Node* root) {
  //   std::vector<Node*> q;
  //   if (root) {
  //     q.push_back(root);
  //   }

  //   while (!q.empty()) {
  //     for (size_t i = 0; i < q.size(); ++i) {
  //       if (i + 1 < q.size()) {
  //         q[i]->next = q[i + 1];
  //       }
  //     }

  //     std::vector<Node*> nq;

  //     for (Node* n : q) {
  //       if (n->left) {
  //         nq.push_back(n->left);
  //       }
  //       if (n->right) {
  //         nq.push_back(n->right);
  //       }
  //     }
  //     std::swap(q, nq);
  //   }
  //   return root;
  // }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
