import std/nre
import std/sequtils
import std/sets
import std/strutils
import std/sugar
import std/tables

type Point = (int, int)

proc manhattan(a, b: Point): int = (a[0]-b[0]).abs + (a[1]-b[1]).abs

proc searchedXInTargetY(sensor: Point, dist, targetY: int): seq[int] =
  if targetY in (sensor[1]-dist)..(sensor[1]+dist):
    if targetY == sensor[1]-dist or targetY == sensor[1]+dist:
      return @[sensor[0]] # diamond top edge or bottom edge
    else:
      let diff = dist-(sensor[1]-targetY).abs
      let x1 = sensor[0]+diff
      let x2 = sensor[0]-diff
      return collect(for x in min(x1, x2)..max(x1, x2): x)
  else: return @[]

proc solve*(input: string, testMode: bool = false): (int, int) =
  let number = re"-?\d+"
  let lines = input.splitLines.mapIt(it.findAll(number).mapIt(it.parseInt))

  let targetY = if not testMode: 2000000 else: 10
  var dists = initTable[Point, int]()
  var beaconsInTargetY: seq[int]
  for c in lines:
    let sensor = (c[0], c[1])
    let beacon = (c[2], c[3])
    dists[sensor] = manhattan(sensor, beacon)
    if beacon[1] == targetY: beaconsInTargetY.add(beacon[0])

  var searchedX: HashSet[int]
  for (sensor, dist) in dists.pairs:
    for x in searchedXInTargetY(sensor, dist, targetY):
      if x notin beaconsInTargetY:
        searchedX.incl(x)
  let answer1 = searchedX.len

  # part 2
  var la = initHashSet[int]()
  var lb = initHashSet[int]()
  for s, d in dists:
    let (x, y) = s
    la.incl(y-x+d+1)
    la.incl(y-x-d-1)
    lb.incl(y+x+d+1)
    lb.incl(y+x-d-1)

  var answer2: int
  for a in la:
    for b in lb:
      let intersection = [(b-a) div 2, (a+b) div 2]
      let limit = if not testMode: 4_000_000 else: 20
      if intersection.allIt(it > 0 and it < limit):
        if dists
          .keys
          .toSeq
          .allIt(manhattan((intersection[0], intersection[1]), it) > dists[it]):
          answer2 = intersection[0] * 4_000_000 + intersection[1]
  return (answer1, answer2)

if isMainModule:
  let answers = solve(stdin.readAll)
  echo answers[0], '\n', answers[1]
