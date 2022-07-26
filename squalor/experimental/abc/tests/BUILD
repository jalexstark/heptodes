# Golden-file testing.

# Run with --diff_lines=10000 or --update_dir=xxxxx if wanting to see output or update goldens.

load(
    "//base_dir/patinon/exploratory/abc/misc:build_defs.bzl",
    "gentest_parser_lexer",
)

licenses(["notice"])

SVT_HTML_NAMES = [
    "svt_test_headings",
    "test3_lists",
    "test4_lists",
    "test5_lists",
    "test6_text_styles",
]

SVT_NAMES = [
    "test0",
    "test1",
] + SVT_HTML_NAMES

# Disable unless and until we decide to merge 3 CommonTokenStream.
# PASS_THROUGH_NAMES = [
#     "test0",
#     "svt_test_headings",
# ]

PVN_HTML_NAMES = [
    "pvn_test_scaffold",
]

PVN_NAMES = [
    "pvn_test0",
] + PVN_HTML_NAMES

exports_files(["svt_inputs/salient-basic.css"])

# simple_copy_file(
#     name = "salient_css",
#     src = "//base_dir/patinon/exploratory/abc/tests:build_defs.bzl",
#     target = "base_dir/patinon/exploratory/abc/tests/goldens/salient-basic.css",
# )

# Salient file test targets.

# Disable unless and until we decide to merge 3 CommonTokenStream.
# [
#     gen_parser_lexer(
#         aaa_gen_name = "%s_lexer_pass_through_gen" % filename,
#         out_dir = "outputs/",
#         src_dir = "svt_inputs/",
#         src_file = "%s.svt" % filename,
#         test_type = "debug_lexer_pass_through",
#     )
#     for filename in PASS_THROUGH_NAMES
# ]
#
# [
#     test_parser_lexer(
#         aaa_test_name = "%s_lexer_pass_through_test" % filename,
#         allow_updates = False,
#         goldens_dir = "svt_inputs/",
#         out_dir = "outputs/",
#         src_file = "%s.svt" % filename,
#         test_type = "debug_lexer_pass_through",
#     )
#     for filename in PASS_THROUGH_NAMES
# ]

[
    gentest_parser_lexer(
        name = "%s_lexer_tokens" % filename,
        aaa_gen_name = "%s_lexer_tokens_gen" % filename,
        aaa_test_name = "%s_lexer_tokens_test" % filename,
        goldens_dir = "goldens/",
        out_dir = "outputs/",
        src_dir = "svt_inputs/",
        src_file = "%s.svt" % filename,
        test_type = "debug_lexer_tokens",
    )
    for filename in SVT_NAMES
]

[
    gentest_parser_lexer(
        name = "%s_inbound_salient" % filename,
        aaa_gen_name = "%s_inbound_salient_gen" % filename,
        aaa_test_name = "%s_inbound_salient_test" % filename,
        goldens_dir = "goldens/",
        out_dir = "outputs/",
        src_dir = "svt_inputs/",
        src_file = "%s.svt" % filename,
        test_type = "debug_inbound_salient",
    )
    for filename in SVT_NAMES
]

[
    gentest_parser_lexer(
        name = "%s_inbound_quarrel" % filename,
        aaa_gen_name = "%s_inbound_quarrel_gen" % filename,
        aaa_test_name = "%s_inbound_quarrel_test" % filename,
        goldens_dir = "goldens/",
        out_dir = "outputs/",
        src_dir = "svt_inputs/",
        src_file = "%s.svt" % filename,
        test_type = "debug_inbound_quarrel",
    )
    for filename in SVT_NAMES
]

# Full Salient test targets.

[
    gentest_parser_lexer(
        name = "%s_salient_to_html" % filename,
        aaa_gen_name = "%s_salient_to_html_gen" % filename,
        aaa_test_name = "%s_salient_to_html_test" % filename,
        goldens_dir = "goldens/",
        out_dir = "outputs/",
        out_extension = "html",
        src_dir = "svt_inputs/",
        src_file = "%s.svt" % filename,
        test_type = "salient_to_html",
    )
    for filename in SVT_HTML_NAMES
]

[
    gentest_parser_lexer(
        name = "%s_salient_reformat" % filename,
        aaa_gen_name = "%s_salient_reformat_gen" % filename,
        aaa_test_name = "%s_salient_reformat_test" % filename,
        goldens_dir = "goldens/",
        out_dir = "outputs/",
        out_extension = "svt",
        src_dir = "svt_inputs/",
        src_file = "%s.svt" % filename,
        test_type = "salient_reformat",
    )
    for filename in SVT_HTML_NAMES
]

# Patinon (Quarrel) file test targets.

# Disable unless and until we decide to merge 3 CommonTokenStream.
# [
#     gentest_parser_lexer(
#         aaa_gen_name = "%s_lexer_pass_through_gen" % filename,
#         aaa_test_name = "%s_lexer_pass_through_test" % filename,
#         goldens_dir = "automatically-reset/",
#         out_dir = "outputs/",
#         src_dir = "pvn_inputs/",
#         src_file = "%s.pvn" % filename,
#         test_type = "debug_lexer_pass_through",
#     )
#     for filename in PVN_NAMES
# ]

[
    gentest_parser_lexer(
        name = "%s_lexer_tokens" % filename,
        aaa_gen_name = "%s_lexer_tokens_gen" % filename,
        aaa_test_name = "%s_lexer_tokens_test" % filename,
        goldens_dir = "goldens/",
        out_dir = "outputs/",
        src_dir = "pvn_inputs/",
        src_file = "%s.pvn" % filename,
        test_type = "debug_lexer_tokens",
    )
    for filename in PVN_NAMES
]

[
    gentest_parser_lexer(
        name = "%s_inbound_quarrel" % filename,
        aaa_gen_name = "%s_inbound_quarrel_gen" % filename,
        aaa_test_name = "%s_inbound_quarrel_test" % filename,
        goldens_dir = "goldens/",
        out_dir = "outputs/",
        src_dir = "pvn_inputs/",
        src_file = "%s.pvn" % filename,
        test_type = "debug_inbound_quarrel",
    )
    for filename in PVN_NAMES
]

[
    gentest_parser_lexer(
        name = "%s_inbound_salient" % filename,
        aaa_gen_name = "%s_inbound_salient_gen" % filename,
        aaa_test_name = "%s_inbound_salient_test" % filename,
        goldens_dir = "goldens/",
        out_dir = "outputs/",
        src_dir = "pvn_inputs/",
        src_file = "%s.pvn" % filename,
        test_type = "debug_inbound_salient",
    )
    for filename in PVN_NAMES
]

# Full Quarrel test targets.

[
    gentest_parser_lexer(
        name = "%s_quarrel_to_html" % filename,
        aaa_gen_name = "%s_quarrel_to_html_gen" % filename,
        aaa_test_name = "%s_quarrel_to_html_test" % filename,
        goldens_dir = "goldens/",
        out_dir = "outputs/",
        out_extension = "html",
        src_dir = "pvn_inputs/",
        src_file = "%s.pvn" % filename,
        test_type = "quarrel_to_html",
    )
    for filename in PVN_HTML_NAMES
]

[
    gentest_parser_lexer(
        name = "%s_quarrel_reformat" % filename,
        aaa_gen_name = "%s_quarrel_reformat_gen" % filename,
        aaa_test_name = "%s_quarrel_reformat_test" % filename,
        goldens_dir = "goldens/",
        out_dir = "outputs/",
        out_extension = "pvn",
        src_dir = "pvn_inputs/",
        src_file = "%s.pvn" % filename,
        test_type = "quarrel_reformat",
    )
    for filename in PVN_HTML_NAMES
]
