cmake_minimum_required(VERSION 3.28)
project(mixr)

set(CMAKE_CXX_STANDARD 17)

add_library(mixr STATIC
        include/mixr/mixr.hpp
        src/Context.cpp
        include/mixr/Context.h
        src/Impl/Impl.cpp
        src/Impl/Impl.h
        include/mixr/AudioFormat.h
        src/AudioBuffer.cpp
        include/mixr/AudioBuffer.h
        src/AudioSource.cpp
        include/mixr/AudioSource.h
)

target_include_directories(mixr PUBLIC include)

add_executable(mixrtest tests/main.cpp)
target_link_libraries(mixrtest PRIVATE mixr)