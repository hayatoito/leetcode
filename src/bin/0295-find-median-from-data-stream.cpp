// @leetup=info id=295 lang=cpp slug=find-median-from-data-stream

#include <algorithm>
#include <cassert>
#include <functional>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class MedianFinder {
 public:
  MedianFinder() = default;

  void addNum(int num) {
    if (small_.empty()) {
      small_.push(num);
    } else {
      if (num < small_.top()) {
        small_.push(num);
      } else {
        large_.push(num);
      }
    }
    if (small_.size() > large_.size() + 1) {
      large_.push(small_.top());
      small_.pop();
    } else if (small_.size() + 1 < large_.size()) {
      small_.push(large_.top());
      large_.pop();
    }
  }

  double findMedian() {
    if (small_.size() == large_.size()) {
      return (small_.top() + large_.top()) / 2.0;
    }
    if (small_.size() == large_.size() + 1) {
      return small_.top();
    } else if (small_.size() + 1 == large_.size()) {
      return large_.top();
    }
    assert(false);
  }

 private:
  std::priority_queue<int> small_;
  std::priority_queue<int, std::vector<int>, std::greater<int>> large_;
};

/**
 * Your MedianFinder object will be instantiated and called as such:
 * MedianFinder* obj = new MedianFinder();
 * obj->addNum(num);
 * double param_2 = obj->findMedian();
 */
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
