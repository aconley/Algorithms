#-------------------
# Test
#-------------------
enable_testing()
set(QUEENS_TEST_NAME ${PROJECT_NAME_STR}_queens_test)
set(LANGFORD_TEST_NAME ${PROJECT_NAME_STR}_langford_test)

include_directories(${GTEST_INCLUDE_DIRS} ${COMMON_INCLUDES})

file(GLOB QUEENS_TEST_SRC_FILES ${PROJECT_SOURCE_DIR}/test/*queens*.cc)
file(GLOB LANGFORD_TEST_SRC_FILES ${PROJECT_SOURCE_DIR}/test/*langford*.cc)

add_executable(${QUEENS_TEST_NAME} ${QUEENS_TEST_SRC_FILES})
add_dependencies(${QUEENS_TEST_NAME} googletest)
target_link_libraries(${QUEENS_TEST_NAME}
        ${GTEST_LIBS_DIR}/libgtest.a
        ${GTEST_LIBS_DIR}/libgtest_main.a)

target_link_libraries(${QUEENS_TEST_NAME} ${CMAKE_THREAD_LIBS_INIT})
add_test(queens ${QUEENS_TEST_NAME})
