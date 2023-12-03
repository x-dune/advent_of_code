#include <cctype>
#include <iostream>
#include <map>
#include <regex>
#include <string>
#include <vector>

#include "../day.h"

void aoc::day01(std::vector<std::string> input) {
  std::map<std::string, char> number_word = {
      {"one", '1'},   {"two", '2'},   {"three", '3'},
      {"four", '4'},  {"five", '5'},  {"six", '6'},
      {"seven", '7'}, {"eight", '8'}, {"nine", '9'},
  };

  std::regex number_word_regex("one|two|three|four|five|six|seven|eight|nine");

  int answer1 = 0;
  int answer2 = 0;

  for (std::string line : input) {
    char left1 = ' ';
    char right1 = ' ';
    char left2 = ' ';
    char right2 = ' ';

    std::string buffer_left = "";
    std::string buffer_right = "";
    for (char c : line) {
      if (left1 == ' ' && isdigit(c)) {
        left1 = c;
      }

      if (left2 == ' ') {
        if (isdigit(c)) {
          left2 = c;
        } else {
          buffer_left += c;
          std::smatch match;
          std::regex_search(buffer_left, match, number_word_regex);
          if (!match.empty()) {
            left2 = number_word[match.str()];
          }
        }
      }

      if (left1 != ' ' && left2 != ' ') {
        break;
      }
    }

    for (int i = (line.size() - 1); i >= 0; i--) {
      char c = line[i];
      if (isdigit(c)) {
        right1 = c;
      }

      if (right2 == ' ') {
        if (isdigit(c)) {
          right2 = c;
        } else {
          buffer_right = c + buffer_right;
          std::smatch match;
          std::regex_search(buffer_right, match, number_word_regex);
          if (!match.empty()) {
            right2 = number_word[match.str()];
          }
        }
      }

      if (right1 != ' ' && right2 != ' ') {
        break;
      }
    }

    answer1 += std::stoi(std::string(1, left1) + right1);
    answer2 += std::stoi(std::string(1, left2) + right2);
  }

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
