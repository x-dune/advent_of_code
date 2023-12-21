#include <iostream>
#include <string>
#include <vector>

#include "../day.h"
#include "../util.h"

namespace {
using Sequence = std::vector<int>;

std::vector<Sequence> parse_input(std::vector<std::string> input) {
  std::vector<Sequence> result;

  for (std::string line : input) {
    Sequence temp;
    auto split = util::resplit(line);
    for (auto c : split) {
      temp.push_back(std::stoi(c));
    }

    result.push_back(temp);
  }

  return result;
}

int is_done(std::vector<Sequence> sequences) {
  Sequence last = sequences[sequences.size() - 1];
  for (auto x : last) {
    if (x != 0) return false;
  }
  return true;
}

std::pair<int, int> extrapolate(Sequence sequence) {
  std::vector<Sequence> sequences = {sequence};
  int i = 0;

  while (!is_done(sequences)) {
    Sequence current = sequences[i];
    Sequence next_sequence;
    for (size_t j = 1; j < current.size(); j++) {
      next_sequence.push_back(current[j] - current[j - 1]);
    }
    sequences.push_back(next_sequence);
    i++;
  }

  int prev = 0;
  int next = 0;

  for (int j = sequences.size() - 2; j >= 0; j--) {
    next += sequences[j][sequences[j].size() - 1];
    prev = sequences[j][0] - prev;
  }

  return {prev, next};
}
}  // namespace

void aoc::day09(std::vector<std::string> input) {
  auto parsed = parse_input(input);

  int answer1 = 0;
  int answer2 = 0;

  for (auto x : parsed) {
    auto extrapolated = extrapolate(x);
    answer1 += extrapolated.second;
    answer2 += extrapolated.first;
  }

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
