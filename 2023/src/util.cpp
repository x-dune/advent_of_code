#include <regex>
#include <string>
#include <vector>

namespace util {
std::vector<std::string> resplit(const std::string &string,
                                 const std::regex &pattern = std::regex{
                                     "\\s+"}) {
  std::sregex_token_iterator iter(string.begin(), string.end(), pattern, -1);
  std::sregex_token_iterator end;
  return {iter, end};
}
std::string join(const std::vector<std::string> &strings,
                 const std::string separator) {
  std::string result = strings[0];
  for (int i = 1; i < strings.size(); i++) {
    result += separator + strings[i];
  }
  return result;
}

}  // namespace util