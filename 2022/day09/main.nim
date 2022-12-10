import std/os
import std/sequtils
import std/sets
import std/strutils

let input = readFile(currentSourcePath.parentDir & "/input.txt")
  .strip
  .splitLines
  .mapIt((it.splitWhitespace[0], parseInt(it.splitWhitespace[1])))

let neighbours = @[(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0,-1), (1, -1)]

proc tailMove(headPosition: (int, int), tailPosition: (int, int)): (int, int) =
  if neighbours.anyIt((tailPosition[0] + it[0], tailPosition[1] + it[1]) == headPosition):
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
var tailPositions = @[(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)]
for _, (dir, amount) in input:
  for _ in 1..amount:
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
    for i, tail in tailPositions:
      let move = tailMove(currentHead, tail)
      if move != (0, 0):
        tailPositions[i] = (tail[0] + move[0], tail[1] + move[1])
      currentHead = tailPositions[i]
    visitedPart1.incl(tailPositions[0])
    visitedPart2.incl(tailPositions[^1])

echo len(visitedPart1), '\n', len(visitedPart2)
