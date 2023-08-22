// @leetup=info id=273 lang=cpp slug=integer-to-english-words

#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  std::string numberToWords(int num) {
    if (num == 0) {
      return "Zero";
    }
    std::vector<std::string> a = helper(static_cast<uint64_t>(num));

    // Join ca with " ".
    std::string ss;
    for (auto it = a.begin(); it != a.end(); it++) {
      if (*it == "") {
        continue;
      }
      if (it != a.begin()) {
        ss += " ";
      }
      ss += *it;
    }
    return ss;
  }

 private:
  std::vector<std::string> helper(uint64_t n) {
    if (n >= 1'000'000'000) {
      std::vector<std::string> a = helper(n / 1'000'000'000);
      a.push_back("Billion");
      std::vector<std::string> b = helper(n % 1'000'000'000);
      a.insert(a.end(), b.begin(), b.end());
      return a;
    }
    if (n >= 1'000'000) {
      std::vector<std::string> a = helper(n / 1'000'000);
      a.push_back("Million");
      std::vector<std::string> b = helper(n % 1'000'000);
      a.insert(a.end(), b.begin(), b.end());
      return a;
    }
    if (n >= 1'000) {
      std::vector<std::string> a = helper(n / 1'000);
      a.push_back("Thousand");
      std::vector<std::string> b = helper(n % 1'000);
      a.insert(a.end(), b.begin(), b.end());
      return a;
    }
    if (n >= 100) {
      std::vector<std::string> a = helper(n / 100);
      a.push_back("Hundred");
      std::vector<std::string> b = helper(n % 100);
      a.insert(a.end(), b.begin(), b.end());
      return a;
    }
    if (n >= 20) {
      std::vector<std::string> a = {tens[n / 10]};
      std::vector<std::string> b = helper(n % 10);
      a.insert(a.end(), b.begin(), b.end());
      return a;
    }
    std::vector<std::string> a = {ones[n]};
    return a;
  }

  std::array<std::string, 20> ones = {
      "",        "One",     "Two",       "Three",    "Four",
      "Five",    "Six",     "Seven",     "Eight",    "Nine",
      "Ten",     "Eleven",  "Twelve",    "Thirteen", "Fourteen",
      "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"};
  std::array<std::string, 10> tens = {
      "",      "",      "Twenty",  "Thirty", "Forty",
      "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
  };
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
