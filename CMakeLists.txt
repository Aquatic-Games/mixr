cmake_minimum_required(VERSION 3.31)
project(mixr)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

add_library(mixr SHARED
        include/mixr/Context.h
        include/mixr/Common.h
        src/mixr/Context.c
        src/mixr/utils/vector.c
        src/mixr/utils/vector.h
        include/mixr/Device.h
        src/mixr/device/deviceimpl.h
        src/mixr/device/device.c
        src/mixr/device/SDL/sdl_device.h
        src/mixr/device/SDL/sdl_device.c
        src/mixr/internal.c
        src/mixr/internal.h
        src/mixr/device/ALSA/alsa_device.c
        src/mixr/device/ALSA/alsa_device.h
)

target_include_directories(mixr PUBLIC include)

add_executable(mixrtest tests/ctest.c)
target_link_libraries(mixrtest PUBLIC mixr)
target_link_libraries(mixrtest PUBLIC SDL2)