#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <iterator>
#include <map>
#include <regex>
#include <set>
#include <string>
#include <vector>

#include "../day.h"

namespace {
using Coord = std::pair<int, int>;

Coord get_start(std::vector<std::string> input) {
  for (size_t y = 0; y < input.size(); y++) {
    for (size_t x = 0; x < input[0].size(); x++) {
      if (input[y][x] == 'S') {
        return {y, x};
      }
    }
  }
  // should be unreachable
  return {0, 0};
}

Coord const dirs[4] = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

int solve1(std::vector<std::string> input, int steps) {
  auto start = get_start(input);
  std::vector<Coord> q = {start};
  const int y_max = input.size();
  const int x_max = input[0].size();

  for (int i = 0; i < steps; i++) {
    std::set<Coord> next_q;
    while (!q.empty()) {
      auto curr = q[q.size() - 1];
      q.pop_back();
      auto& [y, x] = curr;

      for (auto& [dx, dy] : dirs) {
        Coord next = {y + dy, x + dx};

        if (next.first >= 0 && next.first < y_max && next.second >= 0 &&
            next.second < x_max && input[next.first][next.second] != '#') {
          next_q.insert(next);
        }
      }
    }
    q = {next_q.begin(), next_q.end()};
  }
  return q.size();
}

int64_t solve2(std::vector<std::string> input, int steps) {
  // don't undestand how this quadratic solution works
  // https://www.reddit.com/r/adventofcode/comments/18nevo3/comment/keaiiq7/?context=3
  auto start = get_start(input);
  std::vector<Coord> q = {start};
  const int y_max = input.size();
  const int x_max = input[0].size();
  int64_t x = int64_t(steps % input.size());
  int64_t step_checks[3] = {x, x + int64_t(input.size()),
                            x + (int64_t(input.size()) * 2)};
  std::vector<int64_t> xs;

  for (int i = 0; i < steps; i++) {
    std::set<Coord> next_q;
    while (!q.empty()) {
      auto curr = q[q.size() - 1];
      q.pop_back();
      auto& [y, x] = curr;

      for (auto& [dx, dy] : dirs) {
        Coord next = {y + dy, x + dx};
        Coord next_bounded = next;

        if (next.first < 0) {
          next_bounded.first = (y_max + (next.first % y_max)) % y_max;
        } else if (next.first >= y_max) {
          next_bounded.first %= y_max;
        }

        if (next.second < 0) {
          next_bounded.second = (x_max + (next.second % x_max)) % x_max;
        } else if (next.second >= x_max) {
          next_bounded.second %= x_max;
        }

        if (input[next_bounded.first][next_bounded.second] != '#') {
          next_q.insert(next);
        }
      }
    }
    q = {next_q.begin(), next_q.end()};

    auto found =
        std::find(std::begin(step_checks), std::end(step_checks), i + 1);

    if (found != std::end(step_checks)) {
      xs.push_back(q.size());
      if (xs.size() == 3) {
        break;
      }
    }
  }

  int64_t y1 = xs[1] - xs[0];
  int64_t y2 = xs[2] - xs[1];
  int64_t n = steps / int64_t(input.size());

  return xs[0] + y1 * n + ((n * (n - 1)) / 2) * (y2 - y1);
}
}  // namespace

void aoc::day21(std::vector<std::string> input) {
  int answer1 = solve1(input, 64);
  int64_t answer2 = solve2(input, 26501365);

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
