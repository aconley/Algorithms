cmake_minimum_required(VERSION 3.4)
set(PROJECT_NAME_STR ntuples)
project(${PROJECT_NAME_STR} C CXX)

include(CTest)

find_package(Threads REQUIRED)

if(CMAKE_COMPILER_IS_GNUCXX)
    add_definitions(-Wall -std=c++14 -O3 -ansi -Wno-deprecated -pthread)
elseif(CMAKE_CXX_COMPILER_ID MATCHES "Clang")
    add_definitions(-Os -std=c++14)
endif()

project (permutations CXX)

#-------------------
# set common include folder for module
#-------------------
set(COMMON_INCLUDES ${PROJECT_SOURCE_DIR}/include)
set(EXT_PROJECTS_DIR ${PROJECT_SOURCE_DIR}/ext)

#-------------------
# Source code
#-------------------
set(PROJECT_LIB_NAME ${PROJECT_NAME_STR}_impl)

include_directories(${COMMON_INCLUDES})
set(SRC_FILES ${PROJECT_SOURCE_DIR}/src/binarygrey.cc)
add_library(${PROJECT_LIB_NAME} ${SRC_FILES})

add_subdirectory(${EXT_PROJECTS_DIR}/gtest)
add_subdirectory(${EXT_PROJECTS_DIR}/gbench)

#-----------------------
#  test
#-----------------------
enable_testing()
set(PROJECT_TEST_NAME ${PROJECT_NAME_STR}_test)
include_directories(${GTEST_INCLUDE_DIRS} ${COMMON_INCLUDES})

file(GLOB TEST_SRC_FILES ${PROJECT_SOURCE_DIR}/test/*.cc)
add_executable(${PROJECT_TEST_NAME} ${TEST_SRC_FILES})
add_dependencies(${PROJECT_TEST_NAME} googletest)

target_link_libraries(${PROJECT_TEST_NAME}
  ${PROJECT_LIB_NAME}
  ${GTEST_LIBS_DIR}/libgtest.a
  ${GTEST_LIBS_DIR}/libgtest_main.a
)
target_link_libraries(${PROJECT_TEST_NAME} ${CMAKE_THREAD_LIBS_INIT})
add_test(test1 ${PROJECT_TEST_NAME})

#--------------------------
# Benchmarking
#--------------------------
# add_subdirectory(${PROJECT_SOURCE_DIR}/bench)
add_custom_target(bench)
set(PROJECT_BENCH_NAME ${PROJECT_NAME_STR}_bench)
include_directories(${GBENCH_INCLUDE_DIRS} ${COMMON_INCLUDES})
file(GLOB BENCH_SRC_FILES ${PROJECT_SOURCE_DIR}/bench/*.cc)

add_executable(${PROJECT_BENCH_NAME} ${BENCH_SRC_FILES})
add_dependencies(${PROJECT_BENCH_NAME} googlebench)

target_link_libraries(${PROJECT_BENCH_NAME}
        ${PROJECT_LIB_NAME}
        ${GBENCH_LIBS_DIR}/libbenchmark.a)

