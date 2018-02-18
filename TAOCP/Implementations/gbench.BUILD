cc_library(
    name = "gbench",
    srcs = glob(["src/*.cc"],
                exclude = ["src/re_posix.cc", "src/gnuregex.cc"]),
    hdrs = glob(["src/*.h", "include/benchmark/*.h"],
                exclude = ["src/re_posix.h", "src/gnuregex.h"]),
    includes = [
         "include",
    ],
    visibility = ["//visibility:public"],
    copts = [
          "-DHAVE_STD_REGEX"
    ],
)
