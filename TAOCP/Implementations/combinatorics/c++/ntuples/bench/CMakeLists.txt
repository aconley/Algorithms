add_custom_target(bench)
set(PROJECT_BENCH_NAME ${PROJECT_NAME_STR}_bench)
include_directories(${GBENCH_INCLUDE_DIRS} ${COMMON_INCLUDES})
file(GLOB BENCH_SRC_FILES ${PROJECT_SOURCE_DIR}/bench/*.cc)

add_executable(${PROJECT_BENCH_NAME} ${BENCH_SRC_FILES})
add_dependencies(${PROJECT_BENCH_NAME} googlebench)

target_link_libraries(${PROJECT_BENCH_NAME}
        ${PROJECT_LIB_NAME}
        ${GBENCH_LIBS_DIR}/libbenchmark.a)
