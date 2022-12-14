import std/algorithm
import std/json
import std/sequtils
import std/strutils

proc inOrder(left, right: int): int =
  if left == right:
    return 0
  elif left < right:
    return -1 # correct order
  else: return 1 # wrong order

proc compare(left, right: JsonNode): int =
  if left.kind == JInt and right.kind == JInt:
    return inOrder(left.to(int), right.to(int))
  elif left.kind == JArray and right.kind == JInt:
    let arr = newJArray()
    arr.add(right)
    return compare(left, arr)
  elif left.kind == JInt and right.kind == JArray:
    let arr = newJArray()
    arr.add(left)
    return compare(arr, right)
  else: # left.kind == JArray and right.kind == JArray
    for i in 0..min(left.len - 1, right.len - 1):
      let comparison = compare(left[i], right[i])
      if comparison != 0:
        return comparison
    return inOrder(left.len, right.len)

proc solve*(input: string): (int, int) =
  let packets = input.split('\n').filterIt(it != "").mapIt(it.parseJson)
  var answer1 = 0
  for i, pair in packets.distribute(packets.len div 2):
    if compare(pair[0], pair[1]) == -1:
      answer1 += i + 1

  let dividers = @["[[2]]", "[[6]]"].mapIt(it.parseJson)
  var dividerIndices: seq[int] = @[]
  for i, packet in packets.concat(dividers).sorted(compare):
    if dividers.contains(packet):
      dividerIndices.add(i + 1)
    if dividerIndices.len == 2:
      break
  let answer2 = dividerIndices.foldl(a*b)
  return (answer1, answer2)

if isMainModule:
  let answers = solve(stdin.readAll)
  echo answers[0], '\n', answers[1]
