cmake_minimum_required(VERSION 4.2)
project(Slant C)

set(CMAKE_C_STANDARD 99)

file(GLOB SLANT_SRC CONFIGURE_DEPENDS
        include/Slant/*.h
        src/Slant/*.h
        src/Slant/*.c
)

add_library(Slant SHARED ${SLANT_SRC})
