#include <cstddef>
#include <iostream>
#include <ranges>
#include <regex>
#include <string>
#include <vector>

#include "../day.h"
#include "../util.h"

namespace {
struct CubeCount {
  int red;
  int green;
  int blue;

  bool possible(CubeCount other) {
    return other.red >= red && other.green >= green && other.blue >= blue;
  }
};

std::vector<std::vector<CubeCount>> parse_input(
    std::vector<std::string> input) {
  std::vector<std::vector<CubeCount>> result;
  for (auto line : input) {
    auto splits = util::resplit(line, std::regex(": "));
    auto bag_split = util::resplit(splits[1], std::regex("; "));

    auto bag_content = bag_split | std::ranges::views::transform([](auto s) {
                         auto entries = util::resplit(s, std::regex(", "));
                         CubeCount bag = {.red = 0, .green = 0, .blue = 0};
                         for (auto entry : entries) {
                           auto entry_split = util::resplit(entry);
                           auto count = std::stoi(entry_split[0]);

                           if (entry_split[1] == "red") {
                             bag.red = count;
                           } else if (entry_split[1] == "green") {
                             bag.green = count;
                           } else {
                             bag.blue = count;
                           }
                         }
                         return bag;
                       });

    result.push_back({bag_content.begin(), bag_content.end()});
  }
  return result;
};
}  // namespace

void aoc::day02(std::vector<std::string> input) {
  CubeCount possible_bag = {.red = 12, .green = 13, .blue = 14};
  auto games = parse_input(input);
  int answer1 = 0;
  int answer2 = 0;
  for (size_t i = 0; i < games.size(); i++) {
    CubeCount fewest_possible = {.red = 0, .green = 0, .blue = 0};
    bool possible = true;
    for (auto round : games[i]) {
      if (possible && !round.possible(possible_bag)) {
        possible = false;
      }

      if (round.red > fewest_possible.red) {
        fewest_possible.red = round.red;
      }
      if (round.blue > fewest_possible.blue) {
        fewest_possible.blue = round.blue;
      }
      if (round.green > fewest_possible.green) {
        fewest_possible.green = round.green;
      }
    }
    if (possible) {
      answer1 += i + 1;
    }

    answer2 +=
        fewest_possible.red * fewest_possible.green * fewest_possible.blue;
  }

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
