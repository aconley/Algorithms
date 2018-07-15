cc_library(
  name = "langford_lib",
  hdrs = [
    "langford.h",
    "langford_visitors.h",
  ]
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