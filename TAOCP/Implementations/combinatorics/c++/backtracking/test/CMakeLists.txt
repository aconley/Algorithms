#-------------------
# Test
#-------------------
enable_testing()
include_directories(${GTEST_INCLUDE_DIRS} ${COMMON_INCLUDES})

# Queens
set(QUEENS_TEST_NAME ${PROJECT_NAME_STR}_queens_test)
file(GLOB QUEENS_TEST_SRC_FILES ${PROJECT_SOURCE_DIR}/test/*queens*.cc)
add_executable(${QUEENS_TEST_NAME} ${QUEENS_TEST_SRC_FILES})
add_dependencies(${QUEENS_TEST_NAME} googletest)
target_link_libraries(${QUEENS_TEST_NAME}
        ${GTEST_LIBS_DIR}/libgtest.a
        ${GTEST_LIBS_DIR}/libgtest_main.a)
target_link_libraries(${QUEENS_TEST_NAME} ${CMAKE_THREAD_LIBS_INIT})
add_test(queens ${QUEENS_TEST_NAME})

# Langford
set(LANGFORD_TEST_NAME ${PROJECT_NAME_STR}_langford_test)
file(GLOB LANGFORD_TEST_SRC_FILES ${PROJECT_SOURCE_DIR}/test/*langford*.cc)
add_executable(${LANGFORD_TEST_NAME} ${LANGFORD_TEST_SRC_FILES})
add_dependencies(${LANGFORD_TEST_NAME} googletest)
target_link_libraries(${LANGFORD_TEST_NAME}
        ${GTEST_LIBS_DIR}/libgtest.a
        ${GTEST_LIBS_DIR}/libgtest_main.a)
target_link_libraries(${LANGFORD_TEST_NAME} ${CMAKE_THREAD_LIBS_INIT})
add_test(langford ${LANGFORD_TEST_NAME})
