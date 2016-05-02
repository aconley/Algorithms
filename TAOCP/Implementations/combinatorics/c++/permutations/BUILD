cc_library (
	name = "permutations",
	hdrs = ["permutations.h"],
	copts = ["-Os", "-std=c++14"]
)

cc_binary (
	name = "permute_examples",
	srcs = ["permute_examples.cc"],
	deps = [":permutations"],
	copts = ["-std=c++14"]
)

cc_binary (
	name = "permutations_benchmarks",
	srcs = ["permutations_benchmarks.cc"],
	copts = ["-Os", "-std=c++14"],
	deps = ["@benchmark//:main", ":permutations"]
)

cc_test(
    name = "permutations_test",
    srcs = ["permutations_test.cc"],
    copts = ["-Iexternal/gtest/include", "-std=c++14"],
    deps = ["@gtest//:main", ":permutations"],
		size = "small"
)