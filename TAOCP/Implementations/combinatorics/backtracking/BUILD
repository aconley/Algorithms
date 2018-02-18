cc_library(
  name = "nqueens_lib",
  hdrs = [
    "nqueens_array.h",
    "nqueens_basic.h",
    "nqueens_bitwise.h",
    "nqueens_iterative.h",
    "nqueens_walker.h",
  ]
)

cc_library(
  name = "langford_lib",
  hdrs = [
    "langford.h",
    "langford_visitors.h",
  ]
)

cc_test(
  name = "nqueens_test",
  srcs = ["nqueens_test.cc"],
  deps = [
    ":nqueens_lib",
    "@googletest//:gtest_main",
  ],
  size = "small",
)

cc_test(
  name = "langford_test",
  srcs = [
    "langford_test.cc",
    "langford_visitor_test.cc"
  ],
  deps = [
    ":langford_lib",
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

cc_binary(
  name = "langford_bench",
  srcs = ["langford_benchmarks.cc"],
  deps = [
    ":langford_lib",
    "@googlebench//:gbench",
  ]
)

cc_binary(
  name = "langford_balanced",
  srcs = ["langford_balanced.cc"],
  deps = [
    ":langford_lib",
  ]
)