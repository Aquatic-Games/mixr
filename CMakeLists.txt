cmake_minimum_required(VERSION 3.28)
project(mixr)

set(CMAKE_CXX_STANDARD 17)

add_library(mixr STATIC library.cpp
        src/Context.cpp
        include/mixr/Context.h
        include/mixr/mixr.hpp
        src/AudioBuffer.cpp
        include/mixr/AudioBuffer.h
)

target_include_directories(mixr PRIVATE include)
