import std/algorithm
import std/math
import std/sequtils
import std/strutils
import std/tables

let stodChar = {
  '2': 2,
  '1': 1,
  '0': 0,
  '-': -1,
  '=': -2,
}.toTable

# snafu to decimal
proc stod(s: string): int =
  for i, c in s.reversed:
    result += stodChar[c]*(5^i)

let base5Offset = {
  0: "=",
  1: "-",
  2: "0",
  3: "1",
  4: "2",
}.toTable

# decimal to snafu
proc dtos(n: int): string =
  var base5: seq[int]
  var remainder = n + 2 # base 5 with offset +2
  while remainder >= 5:
    base5.add(remainder mod 5)
    remainder = (remainder div 5) + 2
  base5.add(remainder)
  return base5.reversed.mapIt(base5Offset[it]).join

proc solve*(input: string): string =
  let sum = input.strip.splitLines.mapIt(it.stod).foldl(a+b)
  let answer = dtos(sum)
  return answer

if isMainModule:
  echo solve(stdin.readAll)
