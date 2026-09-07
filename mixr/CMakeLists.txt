set(CMAKE_C_STANDARD 99)

file(GLOB_RECURSE MIXR_SRC CONFIGURE_DEPENDS
        include/mixr/*.h
        src/*.c
)

add_library(mixr SHARED ${MIXR_SRC})
target_include_directories(mixr PUBLIC include)