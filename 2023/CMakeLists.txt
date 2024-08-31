cmake_minimum_required(VERSION 3.22)

project(advent_of_code_2023)
set(CMAKE_BUILD_TYPE Release)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_FLAGS "-Wall -Wextra")
set(CMAKE_CXX_FLAGS_DEBUG "-g")
set(CMAKE_CXX_FLAGS_RELEASE "-O3")

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
  src/day/day13.cpp
  src/day/day14.cpp
  src/day/day15.cpp
  src/day/day16.cpp
  src/day/day17.cpp
  src/day/day18.cpp
  src/day/day19.cpp
  src/day/day20.cpp
  src/day/day21.cpp
  src/day/day22.cpp
  src/day/day23.cpp
  src/day/day24.cpp
  src/day/day25.cpp
)
