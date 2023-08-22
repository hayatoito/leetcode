// @leetup=info id=206 lang=cpp slug=reverse-linked-list

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
  ListNode* reverseList(ListNode* head) {
    ListNode* prev = nullptr;
    ListNode* current = head;
    while (current) {
      ListNode* next_node = current->next;
      current->next = prev;
      prev = current;
      current = next_node;

    }
    return prev;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
