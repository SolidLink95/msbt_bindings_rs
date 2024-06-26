cmake_minimum_required(VERSION 3.10)
project(msbt VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(CMAKE_STATIC_LIBRARY_PREFIX "")
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(ABSL_WAITER_MODE 0)

include(CTest)
enable_testing()

file(GLOB TAG_FILES ./src/tags/*.cpp)
add_library(msbt STATIC
    ./src/msbt.cpp
    ./src/include/msbt/msbt.h

    ./src/sections/attribute_section.cpp
    ./src/sections/label_section.cpp
    ./src/sections/text_section.cpp

    ./src/tags.cpp
    ./src/include/msbt/tags.h
    ${TAG_FILES}

    ./src/util.h
)

target_include_directories(msbt PUBLIC ./src/include)
target_include_directories(msbt PRIVATE ./src)

if(MSVC)
    target_compile_options(msbt PRIVATE /W4 /wd4244 /wd4127 /Zc:__cplusplus)
else()
    target_compile_options(msbt PRIVATE -Wall  -fno-plt -fshort-wchar)
endif()

set(BUILD_TESTING ON)
add_subdirectory(./lib/ExtendedIO)
target_link_libraries(msbt
    PUBLIC
    ExtendedIO
)

if(BUILD_TESTING)
    add_subdirectory(tests)
endif()

add_subdirectory(msbt_to_yaml)

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
