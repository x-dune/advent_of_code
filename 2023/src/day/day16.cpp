#include <algorithm>
#include <iostream>
#include <map>
#include <optional>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "../day.h"

namespace {
using IntPair = std::pair<int, int>;
using Beam = std::pair<IntPair, IntPair>;

IntPair const U = {-1, 0};
IntPair const D = {1, 0};
IntPair const L = {0, -1};
IntPair const R = {0, 1};

std::map<IntPair, IntPair> FORWARD_SLASH = {{U, R}, {D, L}, {L, D}, {R, U}};
std::map<IntPair, IntPair> BACK_SLASH = {{U, L}, {D, R}, {L, U}, {R, D}};

std::optional<IntPair> in_bounds(IntPair pos, IntPair offset,
                                 std::vector<std::string> grid) {
  IntPair next = {pos.first + offset.first, pos.second + offset.second};
  if (next.first >= 0 && next.first < grid.size() && next.second >= 0 &&
      next.second < grid[0].size()) {
    return std::optional(next);
  } else {
    return std::nullopt;
  }
}

int beam_traversal(Beam start, std::vector<std::string> grid) {
  std::vector<Beam> q = {start};
  std::set<Beam> seen;

  while (q.size() > 0) {
    auto current = q[q.size() - 1];
    q.pop_back();

    if (seen.contains(current)) {
      continue;
    }

    char current_char = grid[current.first.first][current.first.second];
    std::vector<std::pair<std::optional<IntPair>, IntPair>> next_option;
    if (current_char == '|' && (current.second == R || current.second == L)) {
      next_option = {{in_bounds(current.first, U, grid), U},
                     {in_bounds(current.first, D, grid), D}};
    } else if (current_char == '-' &&
               (current.second == U || current.second == D)) {
      next_option = {{in_bounds(current.first, L, grid), L},
                     {in_bounds(current.first, R, grid), R}};
    } else if (current_char == '/' || current_char == '\\') {
      auto handler = current_char == '/' ? FORWARD_SLASH : BACK_SLASH;
      auto x = handler[current.second];
      next_option = {
          {in_bounds(current.first, handler[current.second], grid),
           handler[current.second]},
      };
    } else {
      next_option = {
          {in_bounds(current.first, current.second, grid), current.second},
      };
    }

    for (auto x : next_option) {
      if (x.first.has_value()) {
        q.push_back({x.first.value(), x.second});
      }
    }
    seen.insert(current);
  }

  std::set<IntPair> seen_pos;
  for (auto &[pos, dir] : seen) {
    seen_pos.insert(pos);
  }

  return seen_pos.size();
}

std::vector<Beam> get_edge_starts(std::vector<std::string> grid) {
  std::vector<Beam> result;
  for (int i = 0; i < grid.size(); i++) {
    result.push_back({{0, i}, D});
    result.push_back({{grid.size() - 1, i}, U});
    result.push_back({{i, 0}, R});
    result.push_back({{i, grid[0].size() - 1}, L});
  }
  return result;
}
}  // namespace

void aoc::day16(std::vector<std::string> input) {
  int answer1 = beam_traversal({{0, 0}, R}, input);
  auto edge_starts = get_edge_starts(input);
  int answer2 = 0;
  for (auto x : edge_starts) {
    answer2 = std::max(answer2, beam_traversal(x, input));
  }
  std::cout << answer1 << '\n' << answer2 << std::endl;
}
