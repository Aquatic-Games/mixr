cmake_minimum_required(VERSION 3.31)
project(mixr)

set(CMAKE_C_STANDARD 1)
set(CMAKE_CXX_STANDARD 17)

add_library(mixr STATIC library.cpp)
