import std/algorithm
import std/sequtils
import std/strutils
import std/tables

type Point = (int, int)
proc `+`(a, b: Point): Point = (a[0]+b[0], a[1]+b[1])
type Grid = Table[Point, char]
let rockPoints = {
  '-': @[(0, 0), (1, 0), (2, 0), (3, 0)],
  '+': @[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
  'J': @[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
  'I': @[(0, 0), (0, 1), (0, 2), (0, 3)],
  'O': @[(0, 0), (1, 0), (0, 1), (1, 1)],
}.toOrderedTable
let rock = rockPoints.keys.toSeq

proc isValid(grid: Grid, pos: Point, rockType: char): bool =
  let points = rockPoints[rockType].mapIt(pos+it)
  let xSorted = points.mapIt(it[0]).sorted
  return xSorted[0] >= 0 and
    xSorted[^1] < 7 and
    points.allIt(not grid.hasKey(it))

proc place(grid: Grid, pos: Point, rockType: char): (Grid, int) =
  var grid = grid
  let points = rockPoints[rockType].mapIt(pos+it)
  for point in points:
    grid[point] = '#'
  let ys = points.mapIt(it[1])
  let localYMax = ys[ys.maxIndex]
  return (grid, localYMax)

# key: (yIncrement, recentGridState) value: stopped
# recentGridState as in normalized points in yMax-100 to yMax

proc solve*(input: string): (int64, int64) =
  let jets = input.strip.toSeq

  var grid = initTable[Point, char]()
  for i in 0..6:
    grid[(i, 0)] = '_' # floor

  var answer1: int64 = 0
  var answer2: int64 = 0
  var stateRecord = initTable[(int, seq[Point]), int]()

  var i = 0
  var stopped = 0
  var yMax = 0
  var curr = (2, 4)
  while true:
    if stopped == 2022 and answer1 == 0:
      answer1 = yMax

    let rockType = rock[stopped mod rock.len]
    var next = curr
    # jet
    if jets[i mod jets.len] == '<':
      next = curr + (-1, 0)
    else:
      next = curr + (1, 0)
    if isValid(grid, next, rockType):
      curr = next
    # drop
    next = curr + (0, -1)
    if isValid(grid, next, rockType):
      curr = next
    else:
      let (nextGrid, localYMax) = place(grid, curr, rockType)
      grid = nextGrid
      let nextYmax = max(yMax, localYMax)
      #region cycle detection
      let yIncrement = nextYmax-yMax
      let recentGridState = grid.keys.toSeq
        .filterIt(it[1] > nextYmax-100)
        .mapIt(it+(0, -nextYmax)).sorted
      let state = (yIncrement, recentGridState)
      if state notin stateRecord:
        stateRecord[state] = stopped
      else:
        # cycle detected
        let cycleStart = stateRecord[state]
        let cycleLength = stopped-cycleStart
        let yIncrements = stateRecord.pairs.toSeq.sortedByIt(it[1]).mapIt(it[0][0])
        let yMaxBeforeCycle = yIncrements[0..cycleStart-1].foldl(a+b)
        let cycleYIncrements = yIncrements[cycleStart..^1]
        let yIncrementPerCycle = cycleYIncrements.foldl(a+b)

        if answer1 == 0:
          let repeatingStopped = 2022-cycleStart
          answer1 = yMaxBeforeCycle +
            ((repeatingStopped div cycleLength) * yIncrementPerCycle) +
            cycleYIncrements[0..(repeatingStopped mod cycleLength)-1].foldl(a+b, 0)

        let repeatingStopped = 1000000000000-cycleStart
        answer2 = yMaxBeforeCycle +
          ((repeatingStopped div cycleLength) * yIncrementPerCycle) +
          cycleYIncrements[0..(repeatingStopped mod cycleLength)-1].foldl(a+b, 0)
        break
      #endregion cycle detection
      yMax = nextYmax
      curr = (2, yMax+4)
      inc stopped
    inc i

  return (answer1, answer2)

if isMainModule:
  let answers = solve(stdin.readAll)
  echo answers[0], '\n', answers[1]
