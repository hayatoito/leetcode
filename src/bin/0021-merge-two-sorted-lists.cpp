// @leetup=info id=21 lang=cpp slug=merge-two-sorted-lists

#include <algorithm>
#include <iostream>
#include <map>
#include <memory>
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
  ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
    ListNode dummy_head;
    ListNode* head = &dummy_head;
    while (list1 || list2) {
      ListNode* next_node;
      if (!list1) {
        next_node = list2;
        list2 = list2->next;
      } else if (!list2) {
        next_node = list1;
        list1 = list1->next;
      } else if (list1->val < list2->val) {
        next_node = list1;
        list1 = list1->next;
      } else {
        next_node = list2;
        list2 = list2->next;
      }

      head->next = next_node;
      head = next_node;
    }
    return dummy_head.next;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
