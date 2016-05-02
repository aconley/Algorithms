cc_library(
    name = "main",
    srcs = glob(
        ["src/*.cc"]
    ),
    hdrs = glob([
        "include/**/*.h",
        "src/*.h"
    ]),
    copts = ["-Iexternal/benchmark/include", "-std=c++14"],
    visibility = ["//visibility:public"],
)