#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

#include "../day.h"

namespace {
using Coord = std::pair<int, int>;
using OffsetToValidTile = std::pair<Coord, std::vector<char>>;

const OffsetToValidTile CONNECT_TO_TOP = {{-1, 0}, {'|', '7', 'F'}};
const OffsetToValidTile CONNECT_TO_BOTTOM = {{1, 0}, {'|', 'L', 'J'}};
const OffsetToValidTile CONNECT_TO_LEFT = {{0, -1}, {'-', 'L', 'F'}};
const OffsetToValidTile CONNECT_TO_RIGHT = {{0, 1}, {'-', 'J', '7'}};

const std::map<char, std::vector<OffsetToValidTile>> TILE_MAPPING = {
    {'S',
     {CONNECT_TO_TOP, CONNECT_TO_BOTTOM, CONNECT_TO_LEFT, CONNECT_TO_RIGHT}},
    {'|', {CONNECT_TO_TOP, CONNECT_TO_BOTTOM}},
    {'-', {CONNECT_TO_LEFT, CONNECT_TO_RIGHT}},
    {'L', {CONNECT_TO_TOP, CONNECT_TO_RIGHT}},
    {'J', {CONNECT_TO_TOP, CONNECT_TO_LEFT}},
    {'7', {CONNECT_TO_BOTTOM, CONNECT_TO_LEFT}},
    {'F', {CONNECT_TO_BOTTOM, CONNECT_TO_RIGHT}},
};

bool is_offset_valid(Coord coord, int y_max, int x_max, std::set<Coord> seen) {
  auto& [y, x] = coord;
  return y >= 0 && y <= y_max && x >= 0 && x <= x_max && !seen.contains(coord);
}

struct TraverseResult {
  int steps;
  std::set<Coord> seen;
  std::vector<std::string> replaced_grid;
};

TraverseResult traverse(std::vector<std::string> grid) {
  Coord start;

  int y_max = grid.size();
  int x_max = grid[0].size();

  for (int y = 0; y < y_max; y++) {
    for (int x = 0; x < x_max; x++) {
      if (grid[y][x] == 'S') {
        start = {y, x};
      }
    }
  }

  std::set<Coord> seen = {start};
  std::vector<std::pair<Coord, char>> q = {{start, 'S'}};
  std::set<Coord> start_offsets;
  int steps = -1;

  while (q.size() > 0) {
    std::vector<std::pair<Coord, char>> next_q;
    for (auto& [coord, c] : q) {
      auto offsets = TILE_MAPPING.at(c);

      for (auto& [offset, cs] : offsets) {
        Coord next_coord = {coord.first + offset.first,
                            coord.second + offset.second};
        if (is_offset_valid(next_coord, y_max, x_max, seen)) {
          char next_c = grid[next_coord.first][next_coord.second];
          if (std::find(cs.begin(), cs.end(), next_c) != cs.end()) {
            if (steps == -1) {
              start_offsets.insert(offset);
            }
            next_q.push_back({next_coord, next_c});
          }
        }
      }
    }
    q = next_q;
    for (auto& [coord, _] : q) {
      seen.insert(coord);
    }
    steps += 1;
  }

  char replaced_start =
      std::find_if(TILE_MAPPING.begin(), TILE_MAPPING.end(), [&](auto x) {
        if (x.first == 'S') {
          return false;
        } else {
          return std::all_of(x.second.begin(), x.second.end(), [&](auto y) {
            return start_offsets.contains(y.first);
          });
        }
      })->first;

  std::vector<std::string> replaced_grid = grid;
  replaced_grid[start.first][start.second] = replaced_start;

  return {steps, seen, replaced_grid};
}

int get_inner(std::vector<std::string> replaced_grid,
              std::set<Coord> loop_coords) {
  bool inside_loop = false;
  int inner = 0;

  for (int y = 0; y < replaced_grid.size(); y++) {
    for (int x = 0; x < replaced_grid[0].size(); x++) {
      if (loop_coords.contains({y, x})) {
        auto c = replaced_grid[y][x];
        if (c == 'F' || c == '7' || c == '|') {
          inside_loop = !inside_loop;
        }
      } else if (inside_loop) {
        inner += 1;
      }
    }
  }

  return inner;
}
};  // namespace

void aoc::day10(std::vector<std::string> input) {
  auto result = traverse(input);
  int answer1 = result.steps;
  int answer2 = get_inner(result.replaced_grid, result.seen);

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
