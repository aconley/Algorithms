cmake_minimum_required(VERSION 3.4)
project(gbench_builder C CXX)
include(ExternalProject)

ExternalProject_Add(googlebench
    GIT_REPOSITORY https://github.com/google/benchmark.git
    CMAKE_ARGS -DCMAKE_ARCHIVE_OUTPUT_DIRECTORY_DEBUG:PATH=DebugLibs
               -DCMAKE_ARCHIVE_OUTPUT_DIRECTORY_RELEASE:PATH=ReleaseLibs
               -Dgtest_force_shared_crt=ON
               -DBUILD_GTEST=ON
     PREFIX "${CMAKE_CURRENT_BINARY_DIR}"
# Disable install step
    INSTALL_COMMAND ""
)

# Specify include dir
ExternalProject_Get_Property(googlebench source_dir)
set(GBENCH_INCLUDE_DIRS ${source_dir}/include PARENT_SCOPE)

# Specify MainTest's link libraries
ExternalProject_Get_Property(googlebench binary_dir)
set(GBENCH_LIBS_DIR ${binary_dir}/src PARENT_SCOPE)