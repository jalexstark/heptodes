# Foundational libraries for graph manipulation and data input.
#
# These do not "know" about rendering data or input fields, but handle generic import.
#
# Manipulation includes preferred methods for graph construction from order-dimension
# posets.

load("//tools/build_defs/proto/cpp:cc_proto_library.bzl", "cc_proto_library")

licenses(["notice"])

package(
    default_hdrs_check = "strict",
    default_visibility = [":zebraix-base-visibility"],
)

exports_files(glob([
    "*.sh",
]))

package_group(
    name = "zebraix-base-visibility",
    packages = [
        "//base_dir/mid_dir/mid_dir/zebraix/...",
    ],
)

proto_library(
    name = "zebraix_graph_proto",
    srcs = ["zebraix_graph.proto"],
    cc_api_version = 2,
)

cc_proto_library(
    name = "zebraix_graph_cc_proto",
    deps = [":zebraix_graph_proto"],
)

cc_library(
    name = "dominance",
    srcs = [
        "dominance.cc",
        "dominance.h",
    ],
    hdrs = [
        "dominance.h",
    ],
    deps = [
        "//base:logging",
    ],
)

cc_library(
    name = "dominance_conversion",
    srcs = [
        "dominance_conversion.cc",
        "dominance_conversion.h",
    ],
    hdrs = [
        "dominance_conversion.h",
    ],
    deps = [
        ":dominance",
        ":zebraix_graph_cc_proto",
    ],
)
