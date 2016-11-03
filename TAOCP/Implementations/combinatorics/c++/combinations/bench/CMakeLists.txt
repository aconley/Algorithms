add_custom_target(bench)
set(COMBINATIONS_PROJECT_BENCH_NAME ${PROJECT_NAME_STR}_bench)
include_directories(${GBENCH_INCLUDE_DIRS} ${COMMON_INCLUDES})
file(GLOB COMBINATIONS_BENCH_SRC_FILES ${PROJECT_SOURCE_DIR}/bench/*combinations*.cc)

add_executable(${COMBINATIONS_PROJECT_BENCH_NAME} ${COMBINATIONS_BENCH_SRC_FILES})
add_dependencies(${COMBINATIONS_PROJECT_BENCH_NAME} googlebench)

target_link_libraries(${COMBINATIONS_PROJECT_BENCH_NAME}
        ${GBENCH_LIBS_DIR}/libbenchmark.a)
