# Tools for rendering graphs as SVG files and the like.

licenses(["notice"])

package(
    default_hdrs_check = "strict",
    default_visibility = [":zebraix-base-visibility"],
)

package_group(
    name = "zebraix-base-visibility",
    packages = [
        "//base_dir/mid_dir/mid_dir/zebraix/misc/...",
        "//base_dir/mid_dir/mid_dir/zebraix/testdata/samples/...",
        "//base_dir/mid_dir/mid_dir/zebraix/tools/...",
    ],
)

cc_library(
    name = "render_structs",
    srcs = [
        "render_structs.h",
    ],
    hdrs = [
        "render_structs.h",
    ],
    deps = [
        "//base_dir/cairo:some_cairo",
        "//base_dir/mid_dir/mid_dir/zebraix/base:zebraix_graph_cc_proto",
    ],
)

cc_library(
    name = "zebraix_layout",
    srcs = [
        "zebraix_layout.cc",
        "zebraix_layout.h",
    ],
    hdrs = [
        "zebraix_layout.h",
    ],
    deps = [
        ":render_structs",
        "//base",
        "//base:logging",
        "//file/base",
        "//net/proto2/public:proto2",
        "//base_dir/absl/container:flat_hash_map",
        "//base_dir/absl/strings:str_format",
        "//base_dir/cairo:some_cairo",
        "//base_dir/mid_dir/mid_dir/zebraix/base:dominance",
        "//base_dir/mid_dir/mid_dir/zebraix/base:dominance_conversion",
        "//base_dir/mid_dir/mid_dir/zebraix/base:zebraix_graph_cc_proto",
        "//base_dir/mid_dir/mid_dir/zebraix/misc:zebraix_misc_proto",
        "//base_dir/mid_dir/mid_dir/zebraix/misc:zebraix_misc_render",
    ],
)

cc_library(
    name = "zebraix_rendersvg",
    srcs = [
        "zebraix_rendersvg.cc",
        "zebraix_rendersvg.h",
    ],
    hdrs = [
        "zebraix_rendersvg.h",
    ],
    deps = [
        ":render_structs",
        ":zebraix_layout",
        "//base:logging",
        "//net/proto2/public:proto2",
        "//base_dir/absl/container:flat_hash_map",
        "//base_dir/cairo:some_cairo",
        "//base_dir/fontconfig",
        "//base_dir/mid_dir/mid_dir/zebraix/base:dominance",
        "//base_dir/mid_dir/mid_dir/zebraix/misc:zebraix_misc_proto",
        "//base_dir/mid_dir/mid_dir/zebraix/misc:zebraix_misc_render",
        "//base_dir/pango:some_pango",
    ],
)
