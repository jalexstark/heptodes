# Functions that are not quite base-level, because they require use of, say,
# rendering structures. They nonetheless are not specific to a renderer, and
# instead apply general rules such as how automatic choices are made in a
# graph context.

licenses(["notice"])

package(
    default_hdrs_check = "strict",
    default_visibility = ["//base_dir/mid_dir/mid_dir/zebraix/base:zebraix-base-visibility"],
)

cc_library(
    name = "zebraix_misc_proto",
    srcs = [
        "misc_proto.cc",
        "misc_proto.h",
    ],
    hdrs = [
        "misc_proto.h",
    ],
    deps = [
        "//base_dir/mid_dir/mid_dir/zebraix/base:zebraix_graph_cc_proto",
    ],
)

cc_library(
    name = "zebraix_misc_render",
    srcs = [
        "misc_render.cc",
        "misc_render.h",
    ],
    hdrs = [
        "misc_render.h",
    ],
    deps = [
        ":zebraix_misc_proto",
        "//base_dir/mid_dir/mid_dir/zebraix/base:dominance",
        "//base_dir/mid_dir/mid_dir/zebraix/base:zebraix_graph_cc_proto",
    ],
)
