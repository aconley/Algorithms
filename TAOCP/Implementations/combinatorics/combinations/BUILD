cc_library(
  name = "combinations_lib",
  hdrs = [
    "combinations.h",
    "combinations_chase.h",
    "combinations_gray.h",
  ]
)

cc_test(
  name = "combinations_test",
  srcs = ["combinations_test.cc"],
  deps = [
    ":combinations_lib",
    "@googletest//:gtest_main",
  ],
  size = "small",
)

cc_binary(
  name = "combinations_bench",
  srcs = ["combinations_bench.cc"],
  deps = [
    ":combinations_lib",
    "@googlebench//:gbench",
  ]
)
