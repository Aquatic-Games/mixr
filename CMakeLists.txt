cmake_minimum_required(VERSION 3.28)
project(mixr)

set(CMAKE_C_STANDARD 99)
set(CMAKE_CXX_STANDARD 17)

add_library(mixr SHARED
        include/mixr/mixr.hpp
        src/Context.cpp
        include/mixr/Context.h
        src/Impl/Impl.cpp
        src/Impl/Impl.h
        include/mixr/Common.h
        src/AudioBuffer.cpp
        include/mixr/AudioBuffer.h
        src/AudioSource.cpp
        include/mixr/AudioSource.h
        src/AudioDevice.cpp
        include/mixr/AudioDevice.h
        src/Device/SdlDevice.cpp
        include/mixr/Device/SdlDevice.h
        include/mixr/mixr.h
        src/C/Mixr.cpp
        src/C/MixrSDL.cpp
        include/mixr/Stream/AudioStream.hpp
        src/Stream/Wav.cpp
        include/mixr/Stream/Wav.hpp
        include/mixr/Stream/AudioStream.h
        src/C/Stream.cpp
        include/mixr/Stream/Wav.h
        src/C/Wav.cpp
        include/mixr/Utils/ADPCM.h
        src/Utils/ADPCM.cpp
)

target_include_directories(mixr PUBLIC include)
#set_property(TARGET mixr PROPERTY POSITION_INDEPENDENT_CODE 1)

add_subdirectory(lib/SDL)
target_link_libraries(mixr PUBLIC SDL2::SDL2-static)

#add_library(mixr SHARED $<TARGET_OBJECTS:mixr-obj>)
#target_include_directories(mixr PUBLIC include)
#target_link_options(mixr PRIVATE -static)
#target_link_libraries(mixr PUBLIC SDL2::SDL2-static)
set(CMAKE_C_FLAGS_RELEASE "${CMAKE_C_FLAGS_RELEASE} -O3 -s")
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O3 -s")

add_executable(mixrtest tests/main.cpp)
target_link_libraries(mixrtest PRIVATE mixr)