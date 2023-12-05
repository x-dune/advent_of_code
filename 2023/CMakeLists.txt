cmake_minimum_required(VERSION 3.22)

project(advent_of_code_2023)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)

add_executable(
  advent_of_code_2023
  src/main.cpp
  src/util.cpp
  src/day/day01.cpp
  src/day/day02.cpp
)
