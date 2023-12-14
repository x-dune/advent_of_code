#include <iostream>
#include <string>
#include <vector>

#include "../day.h"

namespace {
std::vector<std::string> tilt_north(std::vector<std::string> grid) {
  std::vector<std::string> next_grid;

  for (int y = 0; y < grid.size(); y++) {
    next_grid.push_back(std::string(grid.size(), '.'));
  }

  for (int x = 0; x < grid.size(); x++) {
    int current_y = 0;
    for (int y = 0; y < grid.size(); y++) {
      auto current = grid[y][x];
      if (current == 'O') {
        next_grid[current_y][x] = 'O';
        current_y += 1;
      } else if (current == '#') {
        next_grid[y][x] = current;
        current_y = y + 1;
      }
    }
  }
  return next_grid;
}

int get_north_load(std::vector<std::string> grid) {
  int load = 0;
  for (int y = 0; y < grid.size(); y++) {
    for (int x = 0; x < grid.size(); x++) {
      if (grid[y][x] == 'O') {
        load += grid.size() - y;
      }
    }
  }
  return load;
}

std::vector<std::string> rotate_90(std::vector<std::string> grid) {
  std::vector<std::string> next_grid;

  for (int x = 0; x < grid.size(); x++) {
    std::string temp = "";
    for (int y = 0; y < grid.size(); y++) {
      temp.push_back(grid[grid.size() - 1 - y][x]);
    }
    next_grid.push_back(temp);
  }
  return next_grid;
}

std::vector<std::string> spin_cycle(std::vector<std::string> grid) {
  return rotate_90(
      // east
      tilt_north(rotate_90(
          // south
          tilt_north(rotate_90(
              // west
              tilt_north(rotate_90(
                  // north
                  tilt_north(grid))))))));
}

int solve_part2(std::vector<std::string> grid) {
  std::vector<int> north_loads;

  auto current = grid;
  // arbitrarily run 1000 times
  for (int i = 0; i < 1000; i++) {
    current = spin_cycle(current);
    north_loads.push_back(get_north_load(current));
  }

  // detect cycles
  int last = north_loads[north_loads.size() - 1];

  int cycle_length;
  for (int i = 2; i < north_loads.size(); i++) {
    if (last == north_loads[north_loads.size() - i]) {
      cycle_length = i - 1;
      break;
    }
  }
  int remainder = (1000000000 - 1) % cycle_length;
  for (int i = north_loads.size() - 1; i > 0; i--) {
    if (i % cycle_length == remainder) {
      return north_loads[i];
    }
  }

  return cycle_length;
}

}  // namespace

void aoc::day14(std::vector<std::string> input) {
  int answer1 = get_north_load(tilt_north(input));
  int answer2 = solve_part2(input);

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
