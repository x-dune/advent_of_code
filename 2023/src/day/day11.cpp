#include <algorithm>
#include <cstddef>
#include <cstdlib>
#include <iostream>
#include <ranges>
#include <string>
#include <vector>

#include "../day.h"

namespace {
std::vector<std::pair<int, int>> expand_universe(std::vector<std::string> grid,
                                                 int expansion_factor) {
  std::vector<size_t> empty_row_index;
  std::vector<size_t> empty_col_index;

  for (size_t y = 0; y < grid.size(); y++) {
    auto line = grid[y];
    if (std::all_of(line.begin(), line.end(),
                    [](auto e) { return e == '.'; })) {
      empty_row_index.push_back(y);
    }
    auto col =
        grid | std::ranges::views::transform([&](auto e) { return e[y]; });
    if (std::all_of(col.begin(), col.end(), [](auto e) { return e == '.'; })) {
      empty_col_index.push_back(y);
    }
  }

  std::vector<std::pair<int, int>> galaxies;

  int factor = expansion_factor - 1;
  for (size_t y = 0; y < grid.size(); y++) {
    for (size_t x = 0; x < grid[0].size(); x++) {
      char c = grid[y][x];
      if (c == '#') {
        int expand_y = std::ranges::count_if(empty_row_index,
                                             [&](auto e) { return y > e; }) *
                       factor;
        int expand_x = std::ranges::count_if(empty_col_index,
                                             [&](auto e) { return x > e; }) *
                       factor;
        galaxies.push_back({y + expand_y, x + expand_x});
      }
    }
  }

  return galaxies;
}

int64_t get_sum_distances(std::vector<std::pair<int, int>> galaxies) {
  int64_t sum = 0;

  for (auto a : galaxies) {
    for (auto b : galaxies) {
      if (a == b) {
        continue;
      } else {
        sum += std::abs(a.first - b.first) + std::abs(a.second - b.second);
      }
    }
  }

  return sum / 2;
}
}  // namespace

void aoc::day11(std::vector<std::string> input) {
  int64_t answer1 = get_sum_distances(expand_universe(input, 2));
  int64_t answer2 = get_sum_distances(expand_universe(input, 1000000));

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
