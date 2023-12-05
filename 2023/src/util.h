#pragma once

#include <regex>
#include <string>
#include <vector>

namespace util {
std::vector<std::string> resplit(const std::string &string,
                                 const std::regex &pattern = std::regex{
                                     "\\s+"});
}  // namespace util