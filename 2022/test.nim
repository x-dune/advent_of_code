import std/unittest
import std/os

import day01/main as day01
import day02/main as day02
import day03/main as day03
import day04/main as day04
import day05/main as day05


suite "advent of code 2022":
  let pathPrefix = currentSourcePath.parentDir
  
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
