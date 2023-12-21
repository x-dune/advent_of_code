#include <cmath>
#include <cstddef>
#include <iostream>
#include <numeric>
#include <ranges>
#include <string>
#include <vector>

#include "../day.h"
#include "../util.h"

namespace {
std::vector<int> parse_input_part1(std::string s) {
  auto split = util::resplit(s);
  auto result =
      split | std::ranges::views::drop(1) |
      std::ranges::views::transform([](auto x) { return std::stoi(x); });
  return {result.begin(), result.end()};
}

int64_t parse_input_part2(std::string s) {
  auto split = util::resplit(s);
  std::string result;
  for (size_t i = 1; i < split.size(); i++) {
    result += split[i];
  }
  return stol(result);
}

int64_t race(int64_t time, int64_t dist) {
  // quadratic formula method
  int64_t x1 = std::ceil((time + sqrt(std::pow(time, 2) - (4 * dist))) / 2);
  int64_t x2 = std::floor((time - sqrt(std::pow(time, 2) - (4 * dist))) / 2);
  return x1 - x2 - 1;
}
}  // namespace

void aoc::day06(std::vector<std::string> input) {
  auto times = parse_input_part1(input[0]);
  auto dists = parse_input_part1(input[1]);
  auto time = parse_input_part2(input[0]);
  auto dist = parse_input_part2(input[1]);

  std::vector<int> times_dists;
  std::transform(times.begin(), times.end(), dists.begin(),
                 std::back_inserter(times_dists),
                 [](auto a, auto b) { return race(a, b); });

  int answer1 = std::accumulate(times_dists.begin(), times_dists.end(), 1,
                                std::multiplies());
  int64_t answer2 = race(time, dist);

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
