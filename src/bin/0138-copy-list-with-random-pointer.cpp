// @leetup=info id=138 lang=cpp slug=copy-list-with-random-pointer

#include <algorithm>
#include <cassert>
#include <cstddef>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// Definition for a Node.
class Node {
 public:
  int val;
  Node* next;
  Node* random;

  Node(int _val) {
    val = _val;
    next = NULL;
    random = NULL;
  }
};

// @leetup=code

class Solution {
 public:
  Node* copyRandomList(Node* head) {
    std::map<Node*, Node*> p_map;
    p_map[nullptr] = nullptr;

    Node* cur = head;
    Node dummy_head = Node(0);
    Node* new_cur = &dummy_head;
    while (cur) {
      Node* new_node = new Node(cur->val);
      p_map[cur] = new_node;
      cur = cur->next;

      new_cur->next = new_node;
      new_cur = new_node;
    }

    cur = head;
    new_cur = dummy_head.next;
    while (cur) {
      new_cur->random = p_map[cur->random];
      cur = cur->next;
      new_cur = new_cur->next;
    }

    return dummy_head.next;
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
