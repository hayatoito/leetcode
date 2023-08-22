// @leetup=info id=155 lang=cpp slug=min-stack

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <stack>
#include <tuple>
#include <vector>

// @leetup=code

class MinStack {
 public:
  MinStack() {}

  void push(int val) {
    stack_.push(val);
    if (min_stack_.empty()) {
      min_stack_.push(val);
    } else {
      min_stack_.push(std::min(val, min_stack_.top()));
    }
  }

  void pop() {
    stack_.pop();
    min_stack_.pop();
  }

  int top() { return stack_.top(); }

  int getMin() { return min_stack_.top(); }

 private:
  std::stack<int> stack_;
  std::stack<int> min_stack_;
};

/**
 * Your MinStack object will be instantiated and called as such:
 * MinStack* obj = new MinStack();
 * obj->push(val);
 * obj->pop();
 * int param_3 = obj->top();
 * int param_4 = obj->getMin();
 */
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
