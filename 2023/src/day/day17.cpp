#include <iostream>
#include <map>
#include <queue>
#include <string>
#include <tuple>
#include <vector>

#include "../day.h"

namespace {
bool minPQ(std::tuple<int, int, int, int, int> l,
           std::tuple<int, int, int, int, int> r) {
  return std::get<0>(l) > std::get<0>(r);
}

const std::vector<std::pair<int, int>> UDLR = {
    {-1, 0}, {1, 0}, {0, -1}, {0, 1}};
const std::vector<std::pair<int, int>> DURL = {
    {1, 0}, {-1, 0}, {0, 1}, {0, -1}};

int solve(std::vector<std::vector<int>> grid, bool part2) {
  int y_max = grid.size();
  int x_max = grid[0].size();

  std::vector<std::tuple<int, int, int, int, int>> to_q = {{0, 0, 0, -1, 0}};
  std::priority_queue q(to_q.begin(), to_q.end(), minPQ);
  std::map<std::tuple<int, int, int, int>, int> dist;

  while (q.size() > 0) {
    // cost, y, x, dir_index, travelled_in_dir
    auto curr = q.top();
    auto& [c, y, x, d, t] = curr;
    q.pop();
    if (y == y_max - 1 && x == x_max - 1) {
      return c;
    }
    if (dist.find({y, x, d, t}) != dist.end()) {
      continue;
    }
    dist[{y, x, d, t}] = c;
    for (int i = 0; i < UDLR.size(); i++) {
      int next_y = y + UDLR[i].first;
      int next_x = x + UDLR[i].second;
      int next_t = d == i ? t + 1 : 1;
      bool is_valid_turn =
          part2 ? next_t <= 10 && (i == d || t >= 4 || t == 0) : next_t <= 3;
      bool is_not_reverse = d < 0 || DURL[d] != UDLR[i];

      if (next_y >= 0 && next_y < y_max && next_x >= 0 && next_x < x_max &&
          is_not_reverse && is_valid_turn) {
        int next_c = grid[next_y][next_x] + c;
        q.push({next_c, next_y, next_x, i, next_t});
      }
    }
  }
  // should be unreachable
  return -1;
}
}  // namespace

void aoc::day17(std::vector<std::string> input) {
  std::vector<std::vector<int>> grid;
  for (auto line : input) {
    std::vector<int> temp;
    for (auto c : line) {
      temp.push_back(c - '0');
    }
    grid.push_back(temp);
  }

  int answer1 = solve(grid, false);
  int answer2 = solve(grid, true);

  std::cout << answer1 << '\n' << answer2 << std::endl;
}