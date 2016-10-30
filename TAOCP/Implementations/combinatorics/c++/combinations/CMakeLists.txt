cmake_minimum_required(VERSION 3.4)
set(PROJECT_NAME_STR combinations)
project(${PROJECT_NAME_STR} C CXX)

include(CTest)

find_package(Threads REQUIRED)

if(CMAKE_COMPILER_IS_GNUCXX)
    add_definitions(-Wall -std=c++14 -O3 -ansi -Wno-deprecated -pthread)
elseif(CMAKE_CXX_COMPILER_ID MATCHES "Clang")
    add_definitions(-Os -std=c++14)
endif()

project (combinations CXX)

#-------------------
# set common include folder for module
#-------------------
set(COMMON_INCLUDES ${PROJECT_SOURCE_DIR}/include)
set(EXT_PROJECTS_DIR ${PROJECT_SOURCE_DIR}/ext)

add_subdirectory(${EXT_PROJECTS_DIR}/gtest)
add_subdirectory(${EXT_PROJECTS_DIR}/gbench)

# Tests
add_subdirectory(${PROJECT_SOURCE_DIR}/test)

# Benchmarking
# add_subdirectory(${PROJECT_SOURCE_DIR}/bench)
