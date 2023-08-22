// @leetup=info id=232 lang=cpp slug=implement-queue-using-stacks

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <stack>
#include <tuple>
#include <vector>

// @leetup=code

// push: O(1)
// pop/peek: Amortized O(1)

class MyQueue {
 public:
  MyQueue() = default;

  void push(int x) { input.push(x); }

  int pop() {
    int a = peek();
    output.pop();
    return a;
  }

  int peek() {
    // Move input to output.
    if (output.empty()) {
      while (!input.empty()) {
        output.push(input.top());
        input.pop();
      }
    }
    return output.top();
  }

  bool empty() { return input.empty() && output.empty(); }

 private:
  std::stack<int> input;
  std::stack<int> output;
};

/**
 * Your MyQueue object will be instantiated and called as such:
 * MyQueue* obj = new MyQueue();
 * obj->push(x);
 * int param_2 = obj->pop();
 * int param_3 = obj->peek();
 * bool param_4 = obj->empty();
 */
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
