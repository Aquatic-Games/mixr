cmake_minimum_required(VERSION 3.31)
project(mixr)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

option(MX_FLAC "Enable FLAC file loading." ON)

add_library(mixr SHARED
        include/mixr/context.h
        include/mixr/common.h
        src/mixr/context.c
        src/mixr/utils/vector.c
        src/mixr/utils/vector.h
        include/mixr/device.h
        src/mixr/device/deviceimpl.h
        src/mixr/device/device.c
        #src/mixr/device/SDL/sdl_device.h
        #src/mixr/device/SDL/sdl_device.c
        src/mixr/internal.c
        src/mixr/internal.h
        src/mixr/device/ALSA/alsa_device.c
        src/mixr/device/ALSA/alsa_device.h
        include/mixr/stream.h
        src/mixr/stream/stream.c
        src/mixr/stream/wav.c
        src/mixr/stream/stream_internal.h
        src/mixr/stream/vorbis.c
        src/mixr/stream/flac.c
        src/mixr/stream/mp3.c
)

target_include_directories(mixr PUBLIC include)

target_link_libraries(mixr PUBLIC asound)

if (MX_FLAC)
    set(CMAKE_POSITION_INDEPENDENT_CODE ON)
    set(INSTALL_MANPAGES OFF CACHE BOOL "" FORCE)
    add_subdirectory(lib/flac)
    target_link_libraries(mixr PUBLIC FLAC::FLAC)
endif()

add_executable(mixrtest tests/ctest.c)
target_link_libraries(mixrtest PUBLIC mixr)
target_link_libraries(mixrtest PUBLIC SDL2)