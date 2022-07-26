licenses(["notice"])

package_group(
    name = "zebraixsvg-visibility",
    packages = ["//base_dir/mid_dir/mid_dir/zebraix/testdata/samples/..."],
)

package(
    default_hdrs_check = "strict",
    default_visibility = [":zebraixsvg-visibility"],
)

cc_binary(
    name = "zebraix_svg",
    srcs = [
        "zvx_svg.cc",
    ],
    copts = [
        "-DCAIRO_HAS_FT_FONT",
        "-UCAIRO_HAS_FC_FONT",
    ],
    deps = [
        "//base",
        "//file/localfile",  # Actually needed at runtime.
        "//base_dir/absl/flags:flag",
        "//base_dir/absl/flags:parse",
        "//base_dir/absl/strings",
        "//base_dir/mid_dir/mid_dir/zebraix/render:render_structs",
        "//base_dir/mid_dir/mid_dir/zebraix/render:zebraix_layout",
        "//base_dir/mid_dir/mid_dir/zebraix/render:zebraix_rendersvg",
    ],
)
