// @leetup=info id=146 lang=cpp slug=lru-cache

#include <algorithm>
#include <iostream>
#include <list>
#include <map>
#include <set>
#include <tuple>
#include <unordered_map>
#include <vector>

// @leetup=code

// Ref:
// https://leetcode.com/problems/lru-cache/solutions/931834/c-using-unordered-map-hash-and-list-doubly-linked-list-with-basic-inline-comments/?orderBy=most_votes&languageTags=cpp

// map + linked-list solution.

using key = int;

struct Entry {
  int value;
  std::list<key>::iterator it;
};

class LRUCache {
 public:
  LRUCache(int capacity) : capacity_(capacity) {}

  int get(int key) {
    auto it = cache_.find(key);
    if (it == cache_.end()) {
      return -1;
    }

    // `key` is present. Update `list_`.

    // Remove `key` from `list_`.
    // Move key to front.
    list_.erase(it->second.it);
    list_.push_front(key);
    // Make Entry's `it` point to the front.
    it->second.it = list_.begin();

    return it->second.value;
  }

  void put(int key, int value) {
    auto it = cache_.find(key);
    if (it == cache_.end()) {
      if (cache_.size() == capacity_) {
        // remove the oldest element.
        cache_.erase(list_.back());
        list_.pop_back();
      }

      // push-front key.
      list_.push_front(key);
      cache_[key] = { value, list_.begin()};
    } else {
      // key is present. Move key to the front.
      list_.erase(it->second.it);
      list_.push_front(key);

      // Update Entry.
      it->second.value = value;
      it->second.it = list_.begin();
    }
  }

 private:
  std::unordered_map<key, Entry> cache_;
  // Old keys move to back. New onews enter at front.
  std::list<key> list_;
  size_t capacity_;
};

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache* obj = new LRUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
