// @leetup=info id=225 lang=cpp slug=implement-stack-using-queues

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

// push: O(n)
// pop/top: O(1)

class MyStack {
 public:
  MyStack() = default;

  void push(int x) {
    q2.push(x);
    while (!q1.empty()) {
      q2.push(q1.front());
      q1.pop();
    }
    std::swap(q1, q2);
  }

  int pop() {
    int x = top();
    q1.pop();
    return x;
  }

  int top() {
    assert(!empty());
    return q1.front();
  }

  bool empty() { return q1.empty(); }

 private:
  // Output queue. The order is maintained so q1.front() is the element to pop.
  std::queue<int> q1;
  // Temporary queue.
  std::queue<int> q2;
};

/**
 * Your MyStack object will be instantiated and called as such:
 * MyStack* obj = new MyStack();
 * obj->push(x);
 * int param_2 = obj->pop();
 * int param_3 = obj->top();
 * bool param_4 = obj->empty();
 */
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
