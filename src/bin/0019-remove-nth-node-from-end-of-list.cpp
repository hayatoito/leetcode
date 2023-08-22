// @leetup=info id=19 lang=cpp slug=remove-nth-node-from-end-of-list

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

// @leetup=code

class Solution {
 public:
  ListNode* removeNthFromEnd(ListNode* head, int n) {
    ListNode dummy_head(0, head);

    ListNode* a = &dummy_head;
    ListNode* b = &dummy_head;

    // Advance `a`.
    for (int i = 0; i < n; ++i) {
      a = a->next;
    }
    while (true) {
      if (!a->next) {
        b->next = b->next->next;
        return dummy_head.next;
      }
      a = a->next;
      b = b->next;
    }
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
