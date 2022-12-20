import std/algorithm
import std/sequtils
import std/strutils
import std/tables

type Point = (int, int)
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
  let (x, y) = pos
  let points = rockPoints[rockType].mapIt((x+it[0], y+it[1]))
  let xSorted = points.mapIt(it[0]).sorted
  return xSorted[0] >= 0 and
    xSorted[^1] < 7 and
    points.allIt(not grid.hasKey(it))

proc place(grid: Grid, pos: Point, rockType: char): (Grid, int) =
  var grid = grid
  let (x, y) = pos
  let points = rockPoints[rockType].mapIt((x+it[0], y+it[1]))
  for point in points:
    grid[point] = '#'
  let ys = points.mapIt(it[1])
  let localYMax = ys[ys.maxIndex]
  return (grid, localYMax)

# proc draw(grid: Grid) =
  # let ySorted = grid.keys.toSeq.mapIt(it[1]).sorted()
  # let xSorted = grid.keys.toSeq.mapIt(it[0]).sorted()
  # for y in countdown(ySorted[^1],ySorted[0]):
  #   for x in xSorted[0]..xSorted[^1]:
  #     stdout.write(grid.getOrDefault((x,y), '.'))
  #   stdout.write('\n')

# manual cycle detection
# proc longestRepeatingSubstring(s: string): string =
#   let n = s.len

#   var dp = collect(for _ in 0..n: collect(for _ in 0..n: 0))

#   var index = 0
#   var resLen = 0

#   for i in 1..n:
#     for j in i+1..n:
#       if s[i-1] == s[j-1] and dp[i-1][j-1] < j-i:
#         dp[i][j] = dp[i-1][j-1] + 1

#         if dp[i][j] > resLen:
#           index = max(i, index)
#           resLen = dp[i][j]
#       else:
#         dp[i][j] = 0

#   return s[index-resLen..index-1]

# proc repeated(increments: seq[int]): (bool, string) =
#   let joined = increments.join
#   let longest = longestRepeatingSubstring(joined)

#   let isRepeated = longest.len > 20 and
#     longest.len * 2 < joined.len and
#     joined.endsWith(longest) and
#     joined[0..^(longest.len + 1)].endsWith(longest)

#   return (isRepeated, longest)

proc solve*(input: string): (int, int) =
  let jets = input.strip.toSeq

  var grid = initTable[Point, char]()
  for i in 0..6:
    grid[(i, 0)] = '_' # floor

  var i = 0
  var stopped = 0
  var yMax = 0
  var curr = (2, 4)
  # var increments: seq[int]
  while stopped < 2022:
    let rockType = rock[stopped mod rock.len]
    var next = curr
    # jet
    if jets[i mod jets.len] == '<':
      next = (curr[0]-1, curr[1])
    else:
      next = (curr[0]+1, curr[1])
    if isValid(grid, next, rockType):
      curr = next
    # drop
    next = (curr[0], curr[1]-1)
    if isValid(grid, next, rockType):
      curr = next
    else:
      let (nextGrid, localYMax) = place(grid, curr, rockType)
      grid = nextGrid
      # increments.add(max(yMax, localYMax)-yMax)
      yMax = max(yMax, localYMax)
      curr = (2, yMax+4)
      inc stopped
    inc i

  # part2 was done with manual cycle detection
  # TODO add automated part 2
  return (yMax, 0)

if isMainModule:
  let answers = solve(stdin.readAll)
  echo answers[0]
  # echo answers[0], '\n', answers[1]
