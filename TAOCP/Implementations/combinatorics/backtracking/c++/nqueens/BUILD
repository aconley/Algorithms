cc_library(
  name = "nqueens_lib",
  hdrs = [
    "nqueens_visitors.h",
    "nqueens_array.h",
    "nqueens_basic.h",
    "nqueens_bitwise.h",
    "nqueens_bitwise_loop.h",
    "nqueens_constants.h",
    "nqueens_iterative.h",
    "nqueens_walker.h",
  ],
  srcs = [
    "nqueens_basic.cc",
    "nqueens_bitwise_loop.cc",
    "nqueens_constants.cc",
    "nqueens_walker.cc"
  ],
)

cc_test(
  name = "nqueens_test",
  srcs = [
    "nqueens_array_test.cc",
    "nqueens_basic_test.cc",
    "nqueens_bitwise_test.cc",
    "nqueens_bitwise_loop_test.cc",
    "nqueens_walker_test.cc",
  ],
  deps = [
    ":nqueens_lib",
    "@googletest//:gtest_main",
  ],
  size = "small",
)

cc_binary(
  name = "nqueens_bench",
  srcs = ["nqueens_benchmarks.cc"],
  deps = [
    ":nqueens_lib",
    "@googlebench//:gbench",
  ]
)

cc_library(
  name = "nqueens_skiprow_lib",
  srcs = ["nqueens_skiprow.cc"]
)

cc_binary(
  name = "nqueens_skiprow",
  srcs = ["nqueens_skiprow.cc"],
  deps = [
    ":nqueens_skiprow_lib",
  ]
)

cc_library(
  name = "nqueens_hex_lib",
  srcs = ["nqueens_hex.cc"]
)

cc_binary(
  name = "nqueens_hex",
  srcs = ["nqueens_hex.cc"],
  deps = [
    ":nqueens_hex_lib",
  ]
)