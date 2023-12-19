#include <algorithm>
#include <cstdint>
#include <iostream>
#include <map>
#include <numeric>
#include <regex>
#include <string>
#include <vector>

#include "../day.h"
#include "../util.h"

namespace {
struct Rule {
  char part;
  char op;
  int64_t num;
  std::string id;
};

struct Workflow {
  std::vector<Rule> rules;
  std::string fallback;
};

const std::regex open_brace("\\{");
const std::regex comma(",");
const std::regex colon(":");

std::pair<std::map<std::string, Workflow>, std::vector<std::vector<int64_t>>>
parse_input(std::vector<std::string> input) {
  std::map<std::string, Workflow> map;
  std::vector<std::vector<int64_t>> parts;

  bool after_empty = false;
  for (auto line : input) {
    if (line.empty()) {
      after_empty = true;
      continue;
    }

    if (after_empty) {
      std::string without_brace = line.substr(1, line.size() - 2);
      auto split = util::resplit(without_brace, comma);
      std::vector<int64_t> part;

      for (auto s : split) {
        part.push_back(std::stol(s.substr(2)));
      }

      parts.push_back(part);
    } else {
      auto split1 = util::resplit(line, open_brace);
      std::string name = split1[0];
      auto split2 =
          util::resplit(split1[1].substr(0, split1[1].size() - 1), comma);
      std::vector<Rule> rules;
      for (int i = 0; i < split2.size() - 1; i++) {
        char part = split2[i][0];
        char op = split2[i][1];
        auto split3 = util::resplit(split2[i].substr(2), colon);
        int64_t num = std::stol(split3[0]);
        std::string id = split3[1];
        rules.push_back({part, op, num, id});
      }
      map.insert({name, {rules, split2[split2.size() - 1]}});
    }
  }
  return {map, parts};
}

int64_t solve1(std::map<std::string, Workflow> map,
               std::vector<std::vector<int64_t>> parts) {
  int64_t total = 0;
  for (auto part : parts) {
    std::string id = "in";
    while (id != "A" && id != "R") {
      Workflow workflow = map[id];
      std::string next_id = "";
      for (auto& [_part, op, num, _id] : workflow.rules) {
        int part_index = _part == 'x'   ? 0
                         : _part == 'm' ? 1
                         : _part == 'a' ? 2
                                        : 3;
        auto rating = part[part_index];
        if (op == '<') {
          if (rating < num) {
            next_id = _id;
            break;
          }
        } else if (rating > num) {
          next_id = _id;
          break;
        }
      }

      if (next_id.empty()) {
        id = workflow.fallback;
      } else {
        id = next_id;
      }
    }

    if (id == "A") {
      total += std::accumulate(part.begin(), part.end(), 0);
    }
  }
  return total;
}

struct State {
  std::string id;
  int rule_index;
  std::vector<std::pair<int64_t, int64_t>> ranges;
};

int64_t solve2(std::map<std::string, Workflow> map) {
  int64_t total = 0;
  std::vector<State> q = {
      {"in", 0, {{1, 4000}, {1, 4000}, {1, 4000}, {1, 4000}}}};

  while (!q.empty()) {
    State curent = q[q.size() - 1];
    auto& [id, rule_index, ranges] = curent;
    q.pop_back();

    if (id == "A") {
      int64_t temp = 1;
      for (auto range : ranges) {
        temp *= (range.second - range.first + 1);
      }
      total += temp;
    } else if (id == "R") {
      continue;
    } else {
      Workflow workflow = map[id];
      if (rule_index == workflow.rules.size()) {
        q.push_back({workflow.fallback, 0, ranges});
      } else {
        auto& [_part, op, num, _id] = workflow.rules[rule_index];
        int range_index = _part == 'x'   ? 0
                          : _part == 'm' ? 1
                          : _part == 'a' ? 2
                                         : 3;

        auto true_range = ranges;
        auto false_range = ranges;
        if (op == '<') {
          true_range[range_index].second =
              std::min(true_range[range_index].second, num - 1);
          false_range[range_index].first =
              std::max(false_range[range_index].first, num);
        } else {
          true_range[range_index].first =
              std::max(true_range[range_index].first, num + 1);
          false_range[range_index].second =
              std::min(false_range[range_index].second, num);
        }
        q.push_back({_id, 0, true_range});
        q.push_back({id, rule_index + 1, false_range});
      }
    }
  }

  return total;
}
}  // namespace

void aoc::day19(std::vector<std::string> input) {
  auto parsed = parse_input(input);

  int64_t answer1 = solve1(parsed.first, parsed.second);
  int64_t answer2 = solve2(parsed.first);

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
