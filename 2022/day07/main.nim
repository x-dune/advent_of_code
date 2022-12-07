import std/os
import std/sequtils
import std/strutils
import std/sugar
import std/tables

let input = readFile(currentSourcePath.parentDir & "/input.txt")
  .strip
  .split("$ ")[1..^1]

var sizes = initTable[string, int]()
var totalUsedSize = 0

var cwd: seq[string] = @[]
for i, chunk in input:
  let parts = chunk.strip().split('\n')
  if parts[0].startsWith("cd"):
    let dest = parts[0][3..^1]
    if dest == "..":
      cwd.delete(len(cwd) - 1)
    elif dest == "/":
      cwd = @[]
    else:
      cwd.add(dest)
  else:
    let size = parts[1..^1]
      .filter(x => not x.startsWith("dir"))
      .map((x) => parseInt(x.split(' ')[0]))
      .foldl(a+b, 0)
    totalUsedSize += size
    for i in 0..len(cwd) - 1:
      let path = cwd[0..i].join("/")
      if sizes.hasKey(path):
        sizes[path] += size
      else:
        sizes[path] = size

let sizeNeeded = 3_000_0000 - (7_000_0000 - totalUsedSize)

var answer1 = 0
var answer2 = totalUsedSize
for v in sizes.values:
  if v <= 100_000:
    answer1 += v
  if v >= sizeNeeded and v < answer2:
    answer2 = v

echo answer1, '\n', answer2
