import std/os
import std/sequtils
import std/strutils

let input = readFile(currentSourcePath.parentDir & "/input.txt")
  .strip
  .splitLines
  .mapIt(it.mapIt(parseInt($it)))

proc isVisible(y: int, x: int, grid: seq[seq[int]]): bool =
  let current = grid[y][x]
  if grid[y][0..x-1].allIt(it < current) or
    grid[y][x+1..^1].allIt(it < current) or
    grid[0..y-1].mapIt(it[x]).allIt(it < current) or
    grid[y+1..^1].mapIt(it[x]).allIt(it < current):
    return true
  else:
    return false

proc scenicScore(y: int, x: int, grid: seq[seq[int]]): int =
  let current = grid[y][x]
  var top = 0
  var left = 0
  var down = 0
  var right = 0
  for i in 0..y-1:
    top += 1
    if grid[y-1-i][x] >= current:
      break
  for i in 0..x-1:
    left += 1
    if grid[y][x-1-i] >= current:
      break
  for i in y+1..grid.high:
    down += 1
    if grid[i][x] >= current:
      break
  for i in x+1..grid[0].high:
    right += 1
    if grid[y][i] >= current:
      break
  return top * left * down * right

var answer1 = 0
var answer2 = 0

for i, line in input:
  for j, _ in line:
    # part 1
    if i == 0 or j == 0 or i == input.high or j == line.high:
      answer1 += 1
    else:
      if isVisible(i, j, input):
        answer1 += 1
      # part 2
      let currentScenicScore = scenicScore(i, j, input)
      if currentScenicScore > answer2:
        answer2 = currentScenicScore

echo answer1, '\n', answer2
