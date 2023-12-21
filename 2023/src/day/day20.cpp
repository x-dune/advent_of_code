#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <iterator>
#include <map>
#include <regex>
#include <string>
#include <vector>

#include "../day.h"
#include "../util.h"

namespace {
struct Module {
  char type;
  std::vector<std::string> dests;
  bool state;  // for FliFlop: is_on, for Conjunction is_last_high
};
using Map = std::map<std::string, Module>;
const std::regex arrow(" -> ");
const std::regex comma(", ");

std::pair<Map, std::vector<std::string>> parse_input(
    std::vector<std::string> input) {
  Map map;
  std::vector<std::string> broadcaster_dests;
  for (auto line : input) {
    auto split1 = util::resplit(line, arrow);
    auto dests = util::resplit(split1[1], comma);

    if (split1[0] == "broadcaster") {
      broadcaster_dests = dests;
    } else {
      char type = split1[0][0];
      std::string name = split1[0].substr(1);
      map.insert({name, {type, dests, false}});
    }
  }
  return {map, broadcaster_dests};
}

std::vector<std::string> get_periodics(Map map) {
  // theory: there is one Conjuction (A) that outputs to rx
  // in order for A to output low pulse to rx, it needs to have all inputs
  // (Periodics) be high pulse all of the Periodics are Conjuctions, this
  // functions find their name and return them the lcm of the period of each
  // Periodics is the answer to part 2

  auto target = "rx";
  std::vector<std::string> output_to_target;
  for (auto& [k, v] : map) {
    if (v.type == '&' &&
        std::find(v.dests.begin(), v.dests.end(), target) != v.dests.end()) {
      output_to_target.push_back(k);
    }
  }

  std::vector<std::string> periodics;
  if (output_to_target.size() == 1) {
    for (auto& [k, v] : map) {
      if (v.type == '&' && std::find(v.dests.begin(), v.dests.end(),
                                     output_to_target[0]) != v.dests.end()) {
        periodics.push_back(k);
      }
    }
  }

  return periodics;
}

std::pair<int64_t, int64_t> solve(Map map,
                                  std::vector<std::string> broadcaster_dests) {
  std::vector<std::pair<std::string, bool>> q;
  for (auto dest : broadcaster_dests) {
    q.push_back({dest, false});
  }
  auto start_q = q;

  int64_t low = 0;
  int64_t high = 0;
  int i = 0;

  auto periodics = get_periodics(map);
  std::vector<int64_t> periodics_period;
  for (size_t i = 0; i < periodics.size(); i++) {
    periodics_period.push_back(0);
  }

  while (true) {
    i++;
    if (i <= 1000) {
      low++;
    }

    while (!q.empty()) {
      std::vector<std::pair<std::string, bool>> next_q;

      for (auto curr : q) {
        auto& [dest, pulse] = curr;
        if (i <= 1000) {
          if (pulse) {
            high++;
          } else {
            low++;
          }
        }

        if (map.find(dest) != map.end()) {
          Module module = map[dest];

          if (module.type == '%') {
            if (!pulse) {
              for (auto next_dest : module.dests) {
                next_q.push_back({next_dest, !module.state});
              }
              map[dest].state = !module.state;
            }
          } else {
            bool all_high = true;
            for (auto& [k, v] : map) {
              if (std::find(v.dests.begin(), v.dests.end(), dest) !=
                  v.dests.end()) {
                if (!v.state) {
                  all_high = false;
                  break;
                }
              }
            }

            if (!all_high) {
              auto found = std::find(periodics.begin(), periodics.end(), dest);

              if (found != periodics.end()) {
                int j = std::distance(periodics.begin(), found);
                if (periodics_period[j] == 0) {
                  periodics_period[j] = i;
                }

                if (std::all_of(periodics_period.begin(),
                                periodics_period.end(),
                                [](auto x) { return x != 0; })) {
                  goto end;
                }
              }
            }

            for (auto next_dest : module.dests) {
              next_q.push_back({next_dest, !all_high});
            }
            map[dest].state = !all_high;
          }
        }
      }
      q = next_q;
    }

    if (periodics.empty() && i == 1000) {
      break;
    }
    q = start_q;
  }

end:
  return {low * high, util::lcm(periodics_period)};
}
}  // namespace

void aoc::day20(std::vector<std::string> input) {
  auto parsed = parse_input(input);
  auto answer = solve(parsed.first, parsed.second);

  std::cout << answer.first << '\n' << answer.second << std::endl;
}
