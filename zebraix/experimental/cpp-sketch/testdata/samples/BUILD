# Graph and their layouts for testing and for documentation.

load("//tools/build_defs/testing:bzl_library.bzl", "bzl_library")
load(
    "//base_dir/mid_dir/mid_dir/zebraix/testdata/samples:build_defs.bzl",
    "gentest_planar_svg",
)

licenses(["notice"])

Fileset(
    name = "fontfiles",
    out = "fontout",
    entries = [
        FilesetEntry(
            srcdir = "//base_dir/webfonts/apache:BUILD",
            files = [
                "//base_dir/webfonts/apache:all_apache_fonts",
            ],
        ),
    ],
)

SVG_TESTS = [
    # input, output/name, extra_args.
    [
        "sample",
        "sphinx",  # Unused, except as dependency.
        [
            "--generate_sample_graph",
        ],
    ],
    [
        "bridge_three",
        "bridge_three",
        [
            "--vanish_waypoints",
        ],
    ],
    [
        "bridge_three_disorderly",
        "bridge_three_disorderly",
        [
            "--vanish_waypoints",
        ],
    ],
    [
        "bridge_two_a",
        "bridge_two_a",
        [
            "--vanish_waypoints",
        ],
    ],
    [
        "bridge_two_b",
        "bridge_two_b",
        [
            "--vanish_waypoints",
        ],
    ],
    [
        "bridge_two_c",
        "bridge_two_c",
        [
            "--vanish_waypoints",
        ],
    ],
    [
        "bridge_waypoint",
        "bridge_waypoint",
        [],
    ],
    [
        "complicated",
        "complicated",
        [],
    ],
    [
        "config_error",
        "config_error",
        [
            "--dump_inbound_graph=6",
        ],
    ],
    [
        "config_error_ranks",
        "config_error_unadjusted",
        [
            "--label_with_ranks",
        ],
    ],
    [
        "fitness",
        "fitness",
        [
            "--dump_inbound_graph=5",
        ],
    ],
    [
        "fruit_embedding",
        "fruit_embedding",
        [],
    ],
    [
        "fruit_hierarchy",
        "fruit_hierarchy",
        [],
    ],
    [
        "grid_16",
        "grid_16",
        [],
    ],
    [
        "grid_16_on_grid",
        "grid_16_on_grid",
        [],
    ],
    [
        "hierarchy_ranks",
        "fruit_hierarchy",
        [
            "--label_with_ranks",
        ],
    ],
    [
        "inference",
        "inference",
        [],
    ],
    [
        "inference_sink_only",
        "inference_sink_only",
        [],
    ],
    [
        "inference_source_only",
        "inference_source_only",
        [],
    ],
    [
        "long_citrus",
        "long_citrus",
        [],
    ],
    [
        "on_grid",
        "on_grid",
        [
            "--label_with_ranks",
        ],
    ],
    [
        "rank_labels",
        "complicated",
        [
            "--label_with_ranks",
        ],
    ],
    [
        "sample_ticks",
        "config_error",
        [
            "--draw_label_ticks",
        ],
    ],
    [
        "sphinx",
        "sphinx",
        [],
    ],
    [
        "sphinx_ranks",
        "sphinx",
        [
            "--label_with_ranks",
        ],
    ],
    [
        "traffic_dag",
        "traffic_dag",
        [],
    ],
    [
        "traffic_fsm",
        "traffic_fsm",
        [],
    ],
    [
        "cross_complex_01",
        "cross_complex_01",
        [],
    ],
    [
        "cross_simple_01",
        "cross_simple_01",
        [],
    ],
    [
        "cross_simple_02",
        "cross_simple_02",
        [],
    ],
]

[
    gentest_planar_svg(
        name = "%s_svg" % tuplet[0],
        aaa_gen_name = "%s_gen" % tuplet[0],
        aaa_test_name = "%s_test" % tuplet[0],
        extra_args = tuplet[2],
        goldens_dir = "//base_dir/mid_dir/mid_dir/zebraix/doc/goldens/kept",
        out_dir = "outputs/",
        out_file = "%s.svg" % tuplet[0],
        src_dir = "",
        src_file = "%s.pb.txt" % tuplet[1],
    )
    for tuplet in SVG_TESTS
]

bzl_library(
    name = "build_defs_bzl",
    srcs = ["build_defs.bzl"],
    parse_tests = False,
    visibility = ["//visibility:private"],
)
