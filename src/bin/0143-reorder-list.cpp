// @leetup=info id=143 lang=cpp slug=reorder-list

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

// @leetup=code

// Ref: ./0143-reorder-list.py

class Solution {
 public:
  void reorderList(ListNode* head) {
    if (!head) {
      return;
    }

    // Find a middle point.
    ListNode* slow = head;
    ListNode* fast = head;
    while (fast && fast->next) {
      slow = slow->next;
      fast = fast->next->next;
    }

    // Reverses the 2nd half.
    ListNode* prev = nullptr;
    ListNode* curr = slow->next;
    while (curr) {
      ListNode* next_node = curr->next;
      curr->next = prev;
      prev = curr;
      curr = next_node;
    }

    // End the 1st half.
    slow->next = nullptr;

    // Merge the two lists: the 1st half and 2nd half.
    ListNode* a = head;
    ListNode* b = prev;
    while (b) {
      ListNode* next_node = a->next;
      a->next = b;
      a = b;
      b = next_node;
    }

  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
