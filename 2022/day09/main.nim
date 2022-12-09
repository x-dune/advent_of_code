import std/os
import std/sequtils
import std/sets
import std/strutils
import std/sugar

let input = readFile(currentSourcePath.parentDir & "/input.txt")
  .strip
  .split('\n')
  .map((x) => (x.split(' ')[0], parseInt(x.split(' ')[1])))

let neighbourDir = @[(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0,-1), (1, -1)]

proc tailMove(headPosition: (int, int), tailPosition: (int, int)): (int, int) =
  if neighbourDir.any((pos) => (tailPosition[0] + pos[0], tailPosition[1] + pos[1]) == headPosition):
    return (0, 0)
  var yMove = 0
  var xMove = 0
  if headPosition[0] > tailPosition[0]:
    inc yMove
  elif headPosition[0] < tailPosition[0]:
    dec yMove
  if headPosition[1] > tailPosition[1]:
    inc xMove
  elif headPosition[1] < tailPosition[1]:
    dec xMove
  return (yMove, xMove)

var visitedPart1 = toHashSet([(0, 0)])
var visitedPart2 = toHashSet([(0, 0)])
var headPosition = (0, 0)
var tailsPositions = @[(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)]
for _, (dir, amount) in input:
  for i in 1..amount:
    case dir:
      of "U":
        headPosition[0] += 1
      of "D":
        headPosition[0] -= 1
      of "R":
        headPosition[1] += 1
      of "L":
        headPosition[1] -= 1
    var currentHead = headPosition
    for i, tail in tailsPositions:
      let move = tailMove(currentHead, tail)
      if move != (0, 0):
        tailsPositions[i] = (tail[0] + move[0], tail[1] + move[1])
      currentHead = tailsPositions[i]
    visitedPart1.incl(tailsPositions[0])
    visitedPart2.incl(tailsPositions[8])

echo len(visitedPart1), '\n', len(visitedPart2)
