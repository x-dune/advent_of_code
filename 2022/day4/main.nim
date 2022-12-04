import std/os
import std/sequtils
import std/strutils
import std/sugar

let input = readFile(currentSourcePath.parentDir & "/input.txt")
  .strip
  .split("\n")
  .map(x => x.split(',').map(y => y.split('-').map(z => parseInt(z))))

var answer1 = 0
var answer2 = 0
for _, line in input:
  if line[0][0] <= line[1][0] and line[0][1] >= line[1][1]:
    answer1 += 1
  elif line[1][0] <= line[0][0] and line[1][1] >= line[0][1]:
    answer1 += 1

  for n in line[0][0]..line[0][1]:
    if n in line[1][0]..line[1][1]:
      answer2 += 1
      break

echo answer1, '\n', answer2
