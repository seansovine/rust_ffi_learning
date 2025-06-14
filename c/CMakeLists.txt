cmake_minimum_required(VERSION 3.25)
project(TestProgram)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_C_COMPILER clang)
set(CMAKE_CXX_COMPILER clang++)

include_directories(include)

add_library(TestLib STATIC src/test.c)
target_include_directories(
  TestLib
  PUBLIC
  $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include
)

add_executable(TestProgram src/main.c)
target_link_libraries(TestProgram TestLib)
