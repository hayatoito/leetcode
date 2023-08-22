// @leetup=info id=622 lang=cpp slug=design-circular-queue

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class MyCircularQueue {
 public:
  MyCircularQueue(int k) : q_(k) {
    assert(q_.size() == static_cast<size_t>(k));
  }

  bool enQueue(int value) {
    if (isFull()) {
      return false;
    }
    q_[rear_] = value;
    rear_ = (rear_ + 1) % capacity();
    size_ += 1;
    return true;
  }

  bool deQueue() {
    if (isEmpty()) {
      return false;
    }
    size_ -= 1;
    return true;
  }

  int Front() {
    if (isEmpty()) {
      return -1;
    }
    return q_[front_pos()];
  }

  int Rear() {
    if (isEmpty()) {
      return -1;
    }
    return q_[(rear_ + capacity() - 1) % capacity()];
  }

  bool isEmpty() { return size_ == 0; }

  bool isFull() { return size_ == capacity(); }

 private:
  size_t capacity() const { return q_.size(); }
  size_t front_pos() const { return (rear_ + capacity() - size_) % capacity(); }

  std::vector<int> q_;
  size_t size_ = 0;
  size_t rear_ = 0;
};

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * MyCircularQueue* obj = new MyCircularQueue(k);
 * bool param_1 = obj->enQueue(value);
 * bool param_2 = obj->deQueue();
 * int param_3 = obj->Front();
 * int param_4 = obj->Rear();
 * bool param_5 = obj->isEmpty();
 * bool param_6 = obj->isFull();
 */
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  // MyCircularQueue ring(3);
  // bool param_1 = ring.enQueue(1);
  // bool param_2 = ring.deQueue();
  // int param_3 = ring.Front();
  // int param_4 = ring.Rear();
  // bool param_5 = ring.isEmpty();
  // bool param_6 = ring.isFull();
  return 0;
}
