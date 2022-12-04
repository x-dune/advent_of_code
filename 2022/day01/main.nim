import std/algorithm
import std/os
import std/sequtils
import std/strutils
import std/sugar

let input = readFile(currentSourcePath.parentDir & "/input.txt")
  .strip
  .split("\n\n").map(x => x.split('\n').map(y => parseInt(y)))

var caloriesSum = input.map(calories => calories.foldl(a + b))
caloriesSum.sort((a, b) => b - a)
let answer1 = caloriesSum[0]
let answer2 = caloriesSum[0] + caloriesSum[1] + caloriesSum[2]

echo answer1, '\n', answer2
