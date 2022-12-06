import std/os
import std/sets
import std/strutils

let input = readFile(currentSourcePath.parentDir & "/input.txt").strip

var answer1 = 0
var answer2 = 0
var i = 0

while answer1 == 0 or answer2 == 0:
  if answer1 == 0 and len(toHashSet(input[i..i+3])) == 4:
    answer1 = i + 4
  if answer2 == 0 and len(toHashSet(input[i..i+13])) == 14:
    answer2 = i + 14
  i += 1

echo answer1, '\n', answer2
