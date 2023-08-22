// @leetup=info id=160 lang=cpp slug=intersection-of-two-linked-lists

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

struct ListNode {
  int val;
  ListNode* next;
  ListNode(int x) : val(x), next(NULL) {}
};

// @leetup=code

class Solution {
 public:
  ListNode* getIntersectionNode(ListNode* headA, ListNode* headB) {
    ListNode* a = headA;
    ListNode* b = headB;
    int cnt = 0;
    while (a != b) {
      a = a->next;
      b = b->next;
      if (!a) {
        a = headB;
        ++cnt;
        if (cnt == 2) {
          return nullptr;
        }
      }
      if (!b) {
        b = headA;
      }
    }
    return a;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
