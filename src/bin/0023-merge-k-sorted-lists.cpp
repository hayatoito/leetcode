// @leetup=info id=23 lang=cpp slug=merge-k-sorted-lists

#include <algorithm>
#include <iostream>
#include <map>
#include <queue>
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

class Reverse {
 public:
  bool operator()(ListNode* a, ListNode* b) { return a->val > b->val; }
};

class Solution {
 public:
  ListNode* mergeKLists(std::vector<ListNode*>& lists) {
    ListNode dummy_head;
    ListNode* head = &dummy_head;

    std::priority_queue<ListNode*, std::vector<ListNode*>, Reverse> q;
    for (auto n : lists) {
      if (n) {
        q.push(n);
      }
    }

    while (!q.empty()) {
      ListNode* n = q.top();
      q.pop();
      head->next = n;
      head = n;
      if (n->next) {
        q.push(n->next);
      }
    }
    return dummy_head.next;
  }
};
// @leetup=code
int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
