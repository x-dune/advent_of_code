import std/algorithm
import std/deques
import std/sequtils
import std/sets
import std/strutils
import std/sugar
import std/tables

type Pair = (int, int)
proc `+`(a: Pair, b: Pair): Pair = (a[0]+b[0], a[1]+b[1])
type Blizzard = (Pair, char)
type State = (int, Pair) # minute, pos

let blizzardMove = {
  '^': (0, -1),
  'v': (0, 1),
  '<': (-1, 0),
  '>': (1, 0),
}.toTable

let expeditionMoves = [(0, 0), (0, -1), (0, 1), (-1, 0), (1, 0)]

var walls: OrderedSet[Pair]
var playAreaX: seq[int]
var playAreaY: seq[int]
var startGoal: array[2, Pair]

# blizzard table (minute, blizzards)
var blizzardMinutes = initTable[int, seq[Blizzard]]()

proc isValidMove(blizzardPos: OrderedSet[Pair], pos: Pair): bool =
  return (pos == startGoal[0] or pos == startGoal[1]) or
    (pos notin blizzardPos and
    pos[0] in playAreaX and
    pos[1] in playAreaY)

proc getNextBlizzard(blizzards: seq[Blizzard]): seq[Blizzard] =
  for (pos, dir) in blizzards:
    var nextPos = pos + blizzardMove[dir]
    if nextPos notin walls or nextPos == startGoal[0] or nextPos == startGoal[1]:
      result.add((nextPos, dir))
    elif dir == '^':
      result.add(((pos[0], playAreaY[^1]), dir))
    elif dir == 'v':
      result.add(((pos[0], playAreaY[0]), dir))
    elif dir == '<':
      result.add(((playAreaX[^1], pos[1]), dir))
    elif dir == '>':
      result.add(((playAreaX[0], pos[1]), dir))

proc manhattanDistance(a, b: Pair): int = (a[0]-b[0]).abs + (a[1]-b[1]).abs

proc bfs(start, goal: Pair, minute: int, initBlizzards: seq[Blizzard]): int =
  let init = (minute, start)
  var q: Deque[State] = [init].toDeque

  var minMinute = int.high
  var depth = 0
  var seen = [init].toHashSet
  while q.len > 0:
    if depth < q.peekFirst()[0]:
      depth = q.peekFirst()[0]
      blizzardMinutes[depth+1] = getNextBlizzard(blizzardMinutes[depth])
      if q.len > 100: # prune search space if too big, sacrifice accuracy for speed
        q = q.toSeq.sortedByIt(manhattanDistance(it[1], goal))[0..100].toDeque

    let (minute, pos) = q.popFirst()

    if pos == goal:
      if minute < minMinute:
        minMinute = minute
      break

    let nextMinute = minute+1
    let nextBlizzards = blizzardMinutes[nextMinute]
    let nextBlizzardsPos = nextBlizzards.mapIt(it[0]).toOrderedSet

    for move in expeditionMoves:
      let nextPos = pos+move
      if isValidMove(nextBlizzardsPos, nextPos) and
        (nextMinute, nextPos) notin seen:
        q.addLast((nextMinute, nextPos))
        seen.incl((nextMinute, nextPos))

  return minminute

proc solve*(input: string): (int, int) =
  let lines = input.strip.splitLines

  # x, y
  let start = (1, 0)
  let goal = (lines[0].high-1, lines.high)
  var blizzards: seq[Blizzard]

  for y, line in lines:
    for x, c in line:
      if c == '#': walls.incl((x, y))
      elif c != '.': blizzards.add(((x, y), c))

  playAreaX = collect(for x in 1..(lines[0].high-1): x)
  playAreaY = collect(for y in 1..(lines.high-1): y)
  startGoal = [start, goal]

  blizzardMinutes[0] = blizzards
  blizzardMinutes[1] = getNextBlizzard(blizzards)

  let answer1 = bfs(start, goal, 0, blizzards)
  let backToStart = bfs(goal, start, answer1, blizzards)
  let answer2 = bfs(start, goal, backToStart, blizzards)

  return (answer1, answer2)

if isMainModule:
  let answers = solve(stdin.readAll)
  echo answers[0], '\n', answers[1]
