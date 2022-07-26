licenses(["notice"])

package_group(
    name = "zebraix-goldens-visibility",
    packages = [
        "//base_dir/mid_dir/mid_dir/zebraix/testdata/samples/...",
        "//base_dir/mid_dir/mid_dir/zebraix/tools/...",
    ],
)

package(
    default_hdrs_check = "strict",
    default_visibility = [":zebraix-goldens-visibility"],
)

exports_files(glob([
    "*.svg",
]))
