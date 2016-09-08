add_custom_target(bench)
set(QUEENS_PROJECT_BENCH_NAME ${PROJECT_NAME_STR}_queens_bench)
include_directories(${GBENCH_INCLUDE_DIRS} ${COMMON_INCLUDES})
file(GLOB QUEENS_BENCH_SRC_FILES ${PROJECT_SOURCE_DIR}/bench/*queens*.cc)

add_executable(${QUEENS_PROJECT_BENCH_NAME} ${QUEENS_BENCH_SRC_FILES})
add_dependencies(${QUEENS_PROJECT_BENCH_NAME} googlebench)

target_link_libraries(${QUEENS_PROJECT_BENCH_NAME}
        ${GBENCH_LIBS_DIR}/libbenchmark.a)