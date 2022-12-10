import std/os
import std/strutils
import std/unittest

import day01/main as day01
import day02/main as day02
import day03/main as day03
import day04/main as day04
import day05/main as day05
import day06/main as day06
import day07/main as day07
import day08/main as day08
import day09/main as day09
import day10/main as day10

const pathPrefix = currentSourcePath.parentDir

suite "advent of code 2022":
  test "day 1":
    check(day01.solve(readFile(pathPrefix & "/day01/test.txt")) == (24000, 45000))
  test "day 2":
    check(day02.solve(readFile(pathPrefix & "/day02/test.txt")) == (15, 12))
  test "day 3":
    check(day03.solve(readFile(pathPrefix & "/day03/test.txt")) == (157, 70))
  test "day 4":
    check(day04.solve(readFile(pathPrefix & "/day04/test.txt")) == (2, 4))
  test "day 5":
    check(day05.solve(readFile(pathPrefix & "/day05/test.txt")) == ("CMZ", "MCD"))
  test "day 6":
    let testInputs = readFile(pathPrefix & "/day06/tests.txt").strip.splitLines
    const expecteds = [(7, 19), (5, 23), (6, 23), (10, 29), (11, 26)]
    for i, input in testInputs:
      check(day06.solve(input) == expecteds[i])
  test "day 7":
    check(day07.solve(readFile(pathPrefix & "/day07/test.txt")) == (95437, 24933642))
  test "day 8":
    check(day08.solve(readFile(pathPrefix & "/day08/test.txt")) == (21, 8))
  test "day 9":
    check(day09.solve(readFile(pathPrefix & "/day09/test1.txt")) == (13, 1))
    check(day09.solve(readFile(pathPrefix & "/day09/test2.txt"))[1] == 36)
  test "day 10":
    const crtDisplay = @[
      "##..##..##..##..##..##..##..##..##..##..",
      "###...###...###...###...###...###...###.",
      "####....####....####....####....####....",
      "#####.....#####.....#####.....#####.....",
      "######......######......######......####",
      "#######.......#######.......#######.....",
    ]
    check(day10.solve(readFile(pathPrefix & "/day10/test.txt")) == (13140, crtDisplay))
