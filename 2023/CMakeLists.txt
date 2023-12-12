cmake_minimum_required(VERSION 3.22)

project(advent_of_code_2020)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

find_package(Boost 1.74.0 REQUIRED)

if(Boost_FOUND)
  include_directories(${Boost_INCLUDE_DIRS})
endif()

add_executable(
  advent_of_code_2023
  src/main.cpp
  src/util.cpp
  src/day/day01.cpp
  src/day/day02.cpp
  src/day/day03.cpp
  src/day/day04.cpp
  src/day/day05.cpp
  src/day/day06.cpp
  src/day/day07.cpp
  src/day/day08.cpp
  src/day/day09.cpp
  src/day/day10.cpp
  src/day/day11.cpp
  src/day/day12.cpp
)
