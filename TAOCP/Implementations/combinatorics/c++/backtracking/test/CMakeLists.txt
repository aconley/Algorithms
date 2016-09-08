#-------------------
# Test
#-------------------
enable_testing()
set(QUEENS_PROJECT_TEST_NAME ${PROJECT_NAME_STR}_queens_test)
include_directories(${GTEST_INCLUDE_DIRS} ${COMMON_INCLUDES})

file(GLOB QUEENS_TEST_SRC_FILES ${PROJECT_SOURCE_DIR}/test/*queens*.cc)
add_executable(${QUEENS_PROJECT_TEST_NAME} ${QUEENS_TEST_SRC_FILES})
add_dependencies(${QUEENS_PROJECT_TEST_NAME} googletest)

target_link_libraries(${QUEENS_PROJECT_TEST_NAME}
        ${GTEST_LIBS_DIR}/libgtest.a
        ${GTEST_LIBS_DIR}/libgtest_main.a)

target_link_libraries(${QUEENS_PROJECT_TEST_NAME} ${CMAKE_THREAD_LIBS_INIT})

add_test(queens ${QUEENS_PROJECT_TEST_NAME})