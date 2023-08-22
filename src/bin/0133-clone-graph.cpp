// @leetup=info id=133 lang=cpp slug=clone-graph

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <tuple>
#include <vector>

// Definition for a Node.
class Node {
 public:
  int val;
  std::vector<Node*> neighbors;
  Node() {
    val = 0;
    neighbors = std::vector<Node*>();
  }
  Node(int _val) {
    val = _val;
    neighbors = std::vector<Node*>();
  }
  Node(int _val, std::vector<Node*> _neighbors) {
    val = _val;
    neighbors = _neighbors;
  }
};

// @leetup=code

class Solution {
 public:
  Node* cloneGraph(Node* node) {
    if (!node) {
      return nullptr;
    }
    std::map<int, Node*> new_node_map;
    std::set<int> visited;

    std::queue<Node*> old_node_q;
    old_node_q.push(node);

    while (!old_node_q.empty()) {
      Node* old_node = old_node_q.front();
      old_node_q.pop();
      int val = old_node->val;
      if (!visited.insert(val).second) {
        continue;
      }

      Node* new_node;
      auto it = new_node_map.find(val);
      if (it == new_node_map.end()) {
        new_node = new Node(val);
        new_node_map[val] = new_node;
      } else {
        new_node = it->second;
      }

      // Clone neighbors.
      for (Node* old_node_neighbor : old_node->neighbors) {
        old_node_q.push(old_node_neighbor);
        int val = old_node_neighbor->val;
        Node* new_node_neighbor;
        auto it = new_node_map.find(val);
        if (it == new_node_map.end()) {
          new_node_neighbor = new Node(val);
          new_node_map[val] = new_node_neighbor;
        } else {
          new_node_neighbor = it->second;
        }
        new_node->neighbors.push_back(new_node_neighbor);
      }
    }
    // for (auto [i, n] : new_node_map) {
    //   std::cout << "i: " << i << "[";
    //   for (Node* neighbor : n->neighbors) {
    //     std::cout << neighbor->val << ", ";
    //   }
    //   std::cout << "]" << std::endl;
    // }
    return new_node_map[1];
  }
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
