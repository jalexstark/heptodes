load("//base_dir/PROJECT_NAME/bazel:antlr4_cc.bzl", "antlr4_cc_lexer", "antlr4_cc_parser")

package(
    default_visibility = ["//visibility:public"],
    licenses = ["notice"],  # Apache 2.0
)

# Delete?
# antlr4_cc_lexer(
#     name = "common_custom_lexer",
#     src = "CommonCustomLexer.g4",
#     # deps=[
#     #     ":pvn_token",
#     # ],
#     hdrs = [
#         "genres.h",
#         "pvn_token.h",
#         "//base_dir/patinon/exploratory/misc:check_macros.h",
#     ],
#     namespaces = [
#         "patinon",
#         "pvn_parsing",
#     ],
#     # deps = [
#     #     "//base_dir/patinon/exploratory/misc:check_macros.h",
#     # ],
# )

antlr4_cc_lexer(
    name = "pvn_lexer",
    src = "PvnLexer.g4",
    # deps=[
    #     ":common_custom_lexer",
    # ],
    hdrs = [
        "genres.h",
        "pvn_token.h",
        "//base_dir/patinon/exploratory/misc:check_macros.h",
    ],
    imports = [
        "CommonCustomLexer.g4",
        "QvlLexer.g4",
        "SvtLexer.g4",
    ],
    # There is something not quite correct here. It does not matter much which import
    # is requested, but at least one is needed to get the build to work.
    lib_imports = [
        # "CommonCustomLexer.g4",
        # "QvlLexer.g4",
        "SvtLexer.g4",
    ],
    namespaces = [
        "patinon",
        "pvn_parsing",
    ],
    deps = [
        "//base_dir/absl/memory",
        "//base_dir/absl/strings",
    ],
)

antlr4_cc_parser(
    name = "quarrel_parser",
    src = "QvlParser.g4",
    hdrs = [
        "genres.h",
        "pvn_token.h",
        "//base_dir/patinon/exploratory/misc:check_macros.h",
    ],
    imports = [
        "CommonCustomLexer.g4",
        "QvlLexer.g4",
        "SvtLexer.g4",
    ],
    listener = True,
    namespaces = [
        "patinon",
        "pvn_parsing",
    ],
    token_vocab = "PvnLexer.g4",
)

antlr4_cc_parser(
    name = "salient_parser",
    src = "SvtParser.g4",
    hdrs = [
        "genres.h",
        "pvn_token.h",
        "//base_dir/patinon/exploratory/misc:check_macros.h",
    ],
    imports = [
        "CommonCustomLexer.g4",
        "QvlLexer.g4",
        "SvtLexer.g4",
    ],
    listener = True,
    namespaces = [
        "patinon",
        "pvn_parsing",
    ],
    token_vocab = "PvnLexer.g4",
)

cc_library(
    name = "dual_parsing",
    srcs = [
        "customized_lexing.cc",
        "dual_parsing.cc",
        "pvn_token.cc",
    ],
    hdrs = [
        "customized_lexing.h",
        "dual_parsing.h",
        "//base_dir/patinon/exploratory/misc:check_macros.h",
    ],
    copts = [
        "-fexceptions",
    ],
    features = ["-use_header_modules"],  # Incompatible with -fexception
    deps = [
        ":pvn_lexer",
        # ":svn_lexer",
        ":quarrel_parser",
        ":salient_parser",
        "//base_dir/absl/debugging:leak_check",
        "//base_dir/absl/memory",
        "//base_dir/java/antlr4:cc_runtime",
    ],
)
