#include <algorithm>
#include <iostream>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

#include "../day.h"
#include "../util.h"

namespace {
enum HandType {
  high_card = 1,
  one_pair = 2,
  two_pair = 3,
  three_of_a_kind = 4,
  full_house = 5,
  four_of_a_kind = 6,
  five_of_a_kind = 7,
};

std::unordered_map<char, std::string> relative_strength_map = {
    {'A', "E"}, {'K', "D"}, {'Q', "C"}, {'J', "B"}, {'T', "A"},
    {'9', "9"}, {'8', "8"}, {'7', "7"}, {'6', "6"}, {'5', "5"},
    {'4', "4"}, {'3', "3"}, {'2', "2"}};

struct Hand {
  std::string id;
  HandType hand_type;
  HandType hand_type2;
  int bid;

  int relative_strength(bool part2 = false) {
    std::string hex_string = "0x";
    for (auto c : id) {
      if (part2 && c == 'J') {
        hex_string += "1";
        continue;
      }
      hex_string += relative_strength_map[c];
    }
    return std::stoul(hex_string, nullptr, 16);
  }
};

HandType get_hand_type(std::vector<std::pair<char, int>> card_counts,
                       bool part2 = false) {
  auto card_counts_local = card_counts;
  int joker_count = 0;
  if (part2) {
    auto found_joker = std::ranges::find_if(
        card_counts, [](auto x) { return x.first == 'J'; });
    if (found_joker != card_counts.end()) {
      auto i = std::distance(card_counts.begin(), found_joker);
      joker_count = card_counts[i].second;
      std::copy(card_counts.begin(), card_counts.end(),
                card_counts_local.begin());
      card_counts_local.erase(card_counts_local.begin() + i);
    }
  }

  int highest_count = card_counts_local.size() > 0
                          ? card_counts_local[0].second + joker_count
                          : joker_count;

  switch (highest_count) {
    case 5:
      return HandType::five_of_a_kind;
    case 4:
      return HandType::four_of_a_kind;
    case 3:
      if (card_counts_local[1].second == 2) {
        return HandType::full_house;
      } else {
        return HandType::three_of_a_kind;
      }
    case 2:
      if (card_counts_local[1].second == 2) {
        return HandType::two_pair;
      } else {
        return HandType::one_pair;
      }
    default:
      return HandType::high_card;
  }
}

std::vector<Hand> parse_input(std::vector<std::string> input) {
  std::vector<Hand> result;

  for (auto line : input) {
    auto split1 = util::resplit(line);
    int bid = std::stoi(split1[1]);
    std::vector<std::pair<char, int>> card_counts;

    for (auto card : split1[0]) {
      auto card_count = std::ranges::find_if(
          card_counts, [&](auto x) { return x.first == card; });
      if (card_count != card_counts.end()) {
        auto i = std::distance(card_counts.begin(), card_count);
        card_counts[i].second += 1;
      } else {
        card_counts.push_back(std::pair(card, 1));
      }
    }

    std::ranges::sort(card_counts,
                      [](auto a, auto b) { return a.second > b.second; });
    auto hand_type = get_hand_type(card_counts);
    auto hand_type2 = get_hand_type(card_counts, true);
    result.push_back({.id = split1[0],
                      .hand_type = hand_type,
                      .hand_type2 = hand_type2,
                      .bid = bid});
  }

  return result;
};

}  // namespace

void aoc::day07(std::vector<std::string> input) {
  auto cards = parse_input(input);

  std::ranges::sort(cards, [](auto a, auto b) {
    if (a.hand_type == b.hand_type) {
      return a.relative_strength() < b.relative_strength();
    } else {
      return a.hand_type < b.hand_type;
    }
  });

  int answer1 = 0;
  for (int i = 0; i < cards.size(); i++) {
    answer1 += (i + 1) * cards[i].bid;
  }

  std::ranges::sort(cards, [](auto a, auto b) {
    if (a.hand_type2 == b.hand_type2) {
      return a.relative_strength(true) < b.relative_strength(true);
    } else {
      return a.hand_type2 < b.hand_type2;
    }
  });

  int answer2 = 0;
  for (int i = 0; i < cards.size(); i++) {
    answer2 += (i + 1) * cards[i].bid;
  }

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
