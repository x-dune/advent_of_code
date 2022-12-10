import std/sequtils
import std/strutils

let input = readAll(stdin)
  .strip
  .split("\n")
  .mapIt(it.split(',').mapIt(it.split('-').mapIt(parseInt(it))))

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
