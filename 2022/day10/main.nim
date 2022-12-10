import std/sequtils
import std/strutils
import std/sugar

let input = readAll(stdin).strip.splitLines

let temp = collect:
  for _, instruction in input:
    if instruction == "noop": @[instruction]
    # prepend addx instruction with noop to simulate addx taking two cycles
    else: @["noop", instruction]
# flatten seq[seq[string]] to seq[string]
let prependedInput = temp.concat.concat

var x = 1
var xs = newSeq[int]()
var answer2 = collect(for _ in 0..5: "........................................")
for i, instruction in prependedInput:
  let cycle = i + 1
  if cycle in [20, 60, 100, 140, 180, 220]:
    xs.add(cycle * x)

  let spritePositions = [x-1, x, x+1]
  let crtY = i div 40
  let crtX = i mod 40
  if crtX in spritePositions:
    answer2[crtY][crtX] = '#'

  if instruction != "noop":
    x += instruction.splitWhitespace[1].parseInt

let answer1 = xs.foldl(a+b)
echo answer1
# Print answer 2
for _, line in answer2:
  echo line
