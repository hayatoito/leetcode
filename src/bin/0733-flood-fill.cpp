// @leetup=info id=733 lang=cpp slug=flood-fill

#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <tuple>
#include <vector>

// @leetup=code

class Solution {
 public:
  std::vector<std::vector<int>> floodFill(std::vector<std::vector<int>>& image,
                                          int sr,
                                          int sc,
                                          int color) {
    this->image_ = std::move(image);
    color_ = color;
    src_color_ = image_[sr][sc];
    if (color_ == src_color_) {
      return image_;
    }
    dfs(sr, sc);
    return image_;
  }

  void dfs(size_t r, size_t c) {
    if (r < image_.size() && c < image_[0].size() &&
        image_[r][c] == src_color_) {
      image_[r][c] = color_;
      dfs(r, c + 1);
      dfs(r, c - 1);
      dfs(r + 1, c);
      dfs(r - 1, c);
    }
  }

 private:
  std::vector<std::vector<int>> image_;
  int color_;
  int src_color_;
};
// @leetup=code

int main() {
  std::cout << "Hello, world!" << std::endl;
  return 0;
}
