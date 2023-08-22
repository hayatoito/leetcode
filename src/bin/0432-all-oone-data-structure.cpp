// @leetup=info id=432 lang=cpp slug=all-oone-data-structure

#include <algorithm>
#include <cassert>
#include <iostream>
#include <list>
#include <map>
#include <set>
#include <tuple>
#include <unordered_map>
#include <vector>

// @leetup=code

struct Node {
  int count;
  std::unordered_set<std::string> keys;
};

class AllOne {
 public:
  using list_pos = std::list<Node>::iterator;
  AllOne() {}

  void inc(std::string key) {
    auto it = map_.find(key);

    list_pos cur;

    if (it == map_.end()) {
      Node node{0, {key}};
      list_.push_front(node);
      cur = list_.begin();
    } else {
      cur = it->second;
    }

    list_pos next_pos = std::next(cur);

    if (next_pos != list_.end() && cur->count + 1 == next_pos->count) {
      // Add key to the next.
      next_pos->keys.insert(key);
      map_[key] = next_pos;
    } else {
      // Insert a new node.
      Node new_node{cur->count + 1, {key}};
      map_[key] = list_.insert(next_pos, new_node);
    }

    // Remove key from cur's keys
    cur->keys.erase(key);
    if (cur->keys.empty()) {
      list_.erase(cur);
    }
  }

  void dec(std::string key) {
    auto it = map_.find(key);
    assert(it != map_.end());

    list_pos cur = it->second;

    if (cur->count == 1) {
      map_.erase(key);
    } else {
      if (cur != list_.begin()) {
        list_pos prev_pos = std::prev(cur);
        if (cur->count - 1 == prev_pos->count) {
          // Add key to the prev.
          prev_pos->keys.insert(key);
          map_[key] = prev_pos;
        } else {
          // Insert a new node.
          Node new_node{cur->count - 1, {key}};
          map_[key] = list_.insert(cur, new_node);
        }
      } else {
        // Insert a new node at front.
        Node new_node{cur->count - 1, {key}};
        list_.push_front(new_node);
        map_[key] = list_.begin();
      }
    }

    // Remove key from cur's keys.
    cur->keys.erase(key);
    if (cur->keys.empty()) {
      list_.erase(cur);
    }
  }

  std::string getMaxKey() {
    if (list_.empty()) {
      return "";
    }
    return *(list_.back().keys.begin());
  }

  std::string getMinKey() {
    if (list_.empty()) {
      return "";
    }
    return *(list_.front().keys.begin());
  }

 private:
  std::list<Node> list_;
  std::unordered_map<std::string, list_pos> map_;
};

/**
 * Your AllOne object will be instantiated and called as such:
 * AllOne* obj = new AllOne();
 * obj->inc(key);
 * obj->dec(key);
 * string param_3 = obj->getMaxKey();
 * string param_4 = obj->getMinKey();
 */
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
