import std/sequtils
import std/sets
import std/strutils
import std/sugar

const neighbours = @[(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0,-1), (1, -1)]

proc tailMove(head: (int, int), tail: (int, int)): (int, int) =
  if neighbours.anyIt((tail[0] + it[0], tail[1] + it[1]) == head):
    return (0, 0)
  var yMove = 0
  var xMove = 0
  if head[0] > tail[0]:
    inc yMove
  elif head[0] < tail[0]:
    dec yMove
  if head[1] > tail[1]:
    inc xMove
  elif head[1] < tail[1]:
    dec xMove
  return (yMove, xMove)

proc solve*(input: string): (int, int) =
  let movements = input
    .strip
    .splitLines
    .mapIt((it.splitWhitespace[0], parseInt(it.splitWhitespace[1])))

  var firstTailVisited = toHashSet([(0, 0)])
  var lastTailVisited = toHashSet([(0, 0)])
  var head = (0, 0)
  var tails = collect(for _ in 0..8: (0, 0))
  for _, (dir, amount) in movements:
    for _ in 1..amount:
      case dir:
        of "U":
          head[0] += 1
        of "D":
          head[0] -= 1
        of "R":
          head[1] += 1
        of "L":
          head[1] -= 1
      var currentHead = head
      for i, tail in tails:
        let move = tailMove(currentHead, tail)
        if move != (0, 0):
          tails[i] = (tail[0] + move[0], tail[1] + move[1])
        currentHead = tails[i]
      firstTailVisited.incl(tails[0])
      lastTailVisited.incl(tails[^1])
  return (len(firstTailVisited), len(lastTailVisited))

if isMainModule:
  let answers = solve(readAll(stdin))
  echo answers[0], '\n', answers[1]
