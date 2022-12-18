import std/algorithm
import std/sequtils
import std/strutils
import std/sets

const neighbour = [(0, 0, 1), (0, 0, -1), (0, 1, 0), (0, -1, 0), (1, 0, 0), (-1, 0, 0)]
type Point = (int, int, int)
proc `+`(a, b: Point): Point = (a[0]+b[0], a[1]+b[1], a[2]+b[2])

proc solve*(input: string): (int, int) =
  let lava = input
    .strip
    .splitLines
    .mapIt(it.split(',').mapIt(it.parseInt))
    .mapIt((it[0], it[1], it[2]))

  var answer1 = 0
  for _, point in lava:
    answer1 += neighbour.filterIt(not lava.contains(point+it)).len

  # flood fill find external air surrounding lava
  let sortedX = lava.mapIt(it[0]).sorted()
  let sortedY = lava.mapIt(it[1]).sorted()
  let sortedZ = lava.mapIt(it[2]).sorted()
  let start = (sortedX[0]-1, sortedY[0]-1, sortedZ[0]-1)
  var externalAir = [start].toHashSet
  var q = @[start]
  while q.len > 0:
    let curr = q.pop()
    let adjacents = neighbour.mapIt(curr + it)

    if not (curr[0] in sortedX[0]-1..sortedX[^1]+1 and
      curr[1] in sortedY[0]-1..sortedY[^1]+1 and
      curr[2] in sortedZ[0]-1..sortedZ[^1]+1):
      continue

    for _, point in adjacents:
      if not externalAir.contains(point) and
        not lava.contains(point):
        q.add(point)
        externalAir.incl(point)

  # find surface area contacting outside air
  var answer2 = 0
  for _, point in lava:
    answer2 += neighbour.filterIt(externalAir.contains(point + it)).len

  return (answer1, answer2)

if isMainModule:
  let answers = solve(stdin.readAll)
  echo answers[0], '\n', answers[1]
