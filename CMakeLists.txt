cmake_minimum_required(VERSION 3.28)
project(mixr)

set(CMAKE_CXX_STANDARD 17)

add_library(mixr STATIC
        include/mixr/mixr.hpp
        src/Context.cpp
        include/mixr/Context.h)

target_include_directories(mixr PRIVATE include)