cc_library(
  name = "gray_lib",
  hdrs = [
    "binarygray.h",
    "gray.h",
    "looplessgray.h"
  ],
  srcs = [
    "binarygray.cc",
    "gray.cc",
    "looplessgray.cc"
  ]
)

cc_test(
  name = "gray_test",
  srcs = ["gray_test.cc"],
  deps = [
    ":gray_lib",
    "@googletest//:gtest_main",
  ],
  size = "small",
)

cc_binary(
  name = "gray_bench",
  srcs = ["gray_bench.cc"],
  deps = [
    ":gray_lib",
    "@googlebench//:gbench",
  ]
)