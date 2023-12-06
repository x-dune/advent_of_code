#include <cmath>
#include <iostream>
#include <numeric>
#include <ranges>
#include <regex>
#include <string>
#include <vector>

#include "../day.h"
#include "../util.h"

namespace {
struct Card {
  int id;
  int amount = 1;
  std::vector<int> winning;
  std::vector<int> have;

  int matches() {
    return std::accumulate(winning.begin(), winning.end(), 0,
                           [&](auto acc, auto curr) {
                             return acc + !!std::ranges::count(have, curr);
                           });
  }
};

Card parse_input_elem(std::string line) {
  auto split1 = util::resplit(line, std::regex(":\\s+"));
  int card_number = std::stoi(split1[0].substr(5, split1[0].length() - 5));
  auto split2 = util::resplit(split1[1], std::regex("\\s+\\|\\s+"));
  auto card_config_raw =
      split2 | std::ranges::views::transform([](auto x) -> std::vector<int> {
        auto split3 = util::resplit(x);
        auto numbers = split3 | std::ranges::views::transform(
                                    [](auto y) { return std::stoi(y); });
        return {numbers.begin(), numbers.end()};
      });
  std::vector<std::vector<int>> card_config = {card_config_raw.begin(),
                                               card_config_raw.end()};

  return {
      .id = card_number,
      .winning = {card_config[0].begin(), card_config[0].end()},
      .have = {card_config[1].begin(), card_config[1].end()},
  };
};

std::vector<Card> parse_input(std::vector<std::string> input) {
  auto cards = input | std::ranges::views::transform(
                           [](auto x) { return parse_input_elem(x); });
  return {cards.begin(), cards.end()};
}
}  // namespace

void aoc::day04(std::vector<std::string> input) {
  auto cards = parse_input(input);

  int answer1 = 0;
  int answer2 = 0;

  for (int i = 0; i < cards.size(); i++) {
    auto card = cards[i];
    int matches = card.matches();
    if (matches > 0) {
      answer1 += std::pow(2, matches - 1);
      for (int j = 1; j <= matches && (card.id + j) <= cards.size(); j++) {
        cards[card.id + j - 1].amount += card.amount;
      }
    }
    answer2 += card.amount;
  }

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
