// @leetup=info id=237 lang=cpp slug=delete-node-in-a-linked-list

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};

// @leetup=code


class Solution {
public:
    void deleteNode(ListNode* node) {
      ListNode* next = node->next;
      assert(next);
      node->val = next->val;
      node->next = next->next;

      // Optional:
      next->next = nullptr;
      delete(next);
    }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
