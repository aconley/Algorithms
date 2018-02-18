cc_library(
  name = "perms_lib",
  hdrs = ["permutations.h"]
)

cc_test(
  name = "perms_test",
  srcs = ["permutations_test.cc"],
  deps = [
    ":perms_lib",
    "@googletest//:gtest_main",
  ],
  size = "small",
)

cc_binary(
  name = "perms_bench",
  srcs = ["permutations_benchmark.cc"],
  deps = [
    ":perms_lib",
    "@googlebench//:gbench",
  ]
)

cc_binary(
  name = "perms_examples",
  srcs = ["permute_examples.cc"],
  deps = [
    ":perms_lib",
  ]
)
