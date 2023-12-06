#include <algorithm>
#include <iostream>
#include <ranges>
#include <set>
#include <string>
#include <vector>

#include "../day.h"

namespace {
using Coord = std::pair<int, int>;

struct CellNumber {
  int number;
  std::vector<Coord> coords = {};
};

std::vector<Coord> get_adjacent(Coord origin, int y_max, int x_max) {
  std::vector<Coord> potential = {
      {-1, -1}, {-1, 0}, {-1, 1}, {0, -1}, {0, 1}, {1, -1}, {1, 0}, {1, 1},
  };
  auto done = potential | std::ranges::views::transform([&](auto x) -> Coord {
                return {x.first + origin.first, x.second + origin.second};
              }) |
              std::ranges::views::filter([&](auto x) {
                return x.first >= 0 && x.first <= y_max && x.second >= 0 &&
                       x.second <= x_max;
              });

  return {done.begin(), done.end()};
};
}  // namespace

void aoc::day03(std::vector<std::string> input) {
  std::vector<CellNumber> cell_numbers = {};
  std::vector<Coord> symbols = {};
  std::vector<Coord> maybe_gears = {};

  for (int y = 0; y < input.size(); y++) {
    std::vector<Coord> sequence = {};
    std::string number_raw = "";
    for (int x = 0; x < input[0].size(); x++) {
      auto cell = input[y][x];
      Coord coord = {y, x};

      if (std::isdigit(cell)) {
        sequence.push_back(coord);
        number_raw += cell;
        if ((x == input[0].size() || !std::isdigit(input[y][x + 1]))) {
          cell_numbers.push_back(
              {.number = std::stoi(number_raw), .coords = sequence});
          sequence = {};
          number_raw = "";
        }
        continue;
      }

      if (cell != '.') {
        symbols.push_back(coord);
        if (cell == '*') {
          maybe_gears.push_back(coord);
        }
      }
    }
  }

  int answer1 = 0;
  int answer2 = 0;

  for (auto symbol : symbols) {
    auto adjacent = get_adjacent(symbol, input.size(), input[0].size());
    bool maybe_gear = !!std::ranges::count(maybe_gears, symbol);
    std::vector<int> gear_numbers;
    std::set<Coord> seen = {};

    for (auto coord : adjacent) {
      if (!seen.count(coord)) {
        auto cell_number = std::ranges::find_if(cell_numbers, [&](auto x) {
          return !!std::ranges::count(x.coords, coord);
        });
        if (cell_number != cell_numbers.end()) {
          for (auto seen_coord : cell_number->coords) {
            seen.insert(seen_coord);
          }
          answer1 += cell_number->number;
          if (maybe_gear) {
            gear_numbers.push_back(cell_number->number);
          }
        }
      }
    }
    if (gear_numbers.size() == 2) {
      answer2 += gear_numbers[0] * gear_numbers[1];
    }
  }

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
