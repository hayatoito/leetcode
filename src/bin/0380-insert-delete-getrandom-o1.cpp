// @leetup=info id=380 lang=cpp slug=insert-delete-getrandom-o1

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class RandomizedSet {
 public:
  RandomizedSet() = default;

  bool insert(int val) {
    auto it = num_to_index_map_.find(val);
    if (it != num_to_index_map_.end()) {
      return false;
    }
    num_to_index_map_[val] = num_list_.size();
    num_list_.push_back(val);
    return true;
  }

  bool remove(int val) {
    auto it = num_to_index_map_.find(val);
    if (it == num_to_index_map_.end()) {
      return false;
    }
    size_t val_index = it->second;

    int last_value = num_list_.back();
    num_list_[val_index] = last_value;
    num_list_.pop_back();
    num_to_index_map_[last_value] = val_index;

    // Removal should be done at last. e.g. remove is called when size == 1.
    num_to_index_map_.erase(it);
    return true;
  }

  int getRandom() { return num_list_[std::rand() % num_list_.size()]; }

 private:
  std::map<int, size_t> num_to_index_map_;
  std::vector<int> num_list_;
};

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * RandomizedSet* obj = new RandomizedSet();
 * bool param_1 = obj->insert(val);
 * bool param_2 = obj->remove(val);
 * int param_3 = obj->getRandom();
 */
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
