import std/os
import std/strutils

let input = readFile(currentSourcePath.parentDir & "/test_input.txt").strip

for i, _ in input:
  let chars = input[i..i+3]
  if chars.rfind(input[i]) == 0 and chars.rfind(input[i + 1]) == 1 and
      chars.rfind(input[i + 2]) == 2 and chars.rfind(input[i + 3]) == 3:
    echo i + 4
    break

for i, _ in input:
  let chars = input[i..i+13]
  if chars.rfind(input[i]) == 0 and chars.rfind(input[i + 1]) == 1 and
      chars.rfind(input[i + 2]) == 2 and chars.rfind(input[i + 3]) == 3 and
      chars.rfind(input[i + 4]) == 4 and chars.rfind(input[i + 5]) == 5 and
      chars.rfind(input[i + 6]) == 6 and chars.rfind(input[i + 7]) == 7 and
      chars.rfind(input[i + 8]) == 8 and chars.rfind(input[i + 9]) == 9 and
      chars.rfind(input[i + 10]) == 10 and chars.rfind(input[i + 11]) == 11 and
      chars.rfind(input[i + 12]) == 12 and chars.rfind(input[i + 13]) == 13:
    echo i + 14
    break
