package(
    default_visibility = ["//visibility:public"],
    licenses = ["notice"],  # Apache 2.0
)

cc_library(
    name = "coarse_listeners",
    srcs = [
        "skim/coarse_listeners.cc",
        "skim/summarizing.cc",
    ],
    hdrs = [
        # BEGIN Not sure if needed.
        "AltMarkdownPathBaseVisitor.h",
        "AltMarkdownPathLexer.h",
        "AltMarkdownPathParser.h",
        "AltMarkdownPathVisitor.h",
        # END.
        "//base_dir/patinon/exploratory/misc:check_macros.h",
        "skim/enhanced_parse_tree_property.h",
        "skim/coarse_listeners.h",
        "skim/summarizing.h",
    ],
    copts = [
        "-fexceptions",
    ],
    features = ["-use_header_modules"],  # Incompatible with -fexception
    deps = [
        "//base_dir/absl/container:flat_hash_map",
        "//base_dir/absl/container:flat_hash_set",
        "//base_dir/absl/container:node_hash_map",
        "//base_dir/absl/flags:flag",
        "//base_dir/absl/memory",
        "//base_dir/absl/strings",
        "//base_dir/absl/strings:str_format",
        "//base_dir/java/antlr4:cc_runtime",
        "//base_dir/patinon/exploratory/abc/grammys:dual_parsing",
        "//base_dir/patinon/exploratory/abc/grammys:pvn_lexer",
        "//base_dir/patinon/exploratory/abc/grammys:quarrel_parser",
        "//base_dir/patinon/exploratory/abc/grammys:salient_parser",
        "//file/localfile",
        "//strings",
    ],
)

cc_library(
    name = "reformation",
    srcs = [
        "format/reform_handlers.cc",
        "format/reform_listeners.cc",
        "format/styling_data.cc",
    ],
    hdrs = [
        "format/base_handler.h",
        "format/reform_handlers.h",
        "format/reform_listeners.h",
    ],
    deps = [
        ":coarse_listeners",
        "//base_dir/absl/base:core_headers",
        "//base_dir/patinon/exploratory/abc/grammys:dual_parsing",
        "//base_dir/patinon/exploratory/abc/grammys:pvn_lexer",
        "//base_dir/patinon/exploratory/abc/grammys:quarrel_parser",
        "//base_dir/patinon/exploratory/abc/grammys:salient_parser",
        "//strings",
        "//base_dir/absl/container:flat_hash_map",
        "//base_dir/absl/memory",
        "//base_dir/absl/strings",
        "//base_dir/absl/strings:str_format",
        "//base_dir/java/antlr4:cc_runtime",
        # "//base_dir/java/antlr4/v4_7_1/Cpp/src:antlr4_runtime",
    ],
)

cc_binary(
    name = "reform",
    srcs = [
        "format/reform.cc",
        "format/reform_utils.cc",
        "format/reform_utils.h",
    ],
    # hdrs = [
    #     "format/reform_listeners.h",
    # ],
    deps = [
        ":coarse_listeners",
        ":reformation",
        "//base",
        "//base_dir/patinon/exploratory/abc/grammys:dual_parsing",
        "//base_dir/patinon/exploratory/abc/grammys:pvn_lexer",
        "//base_dir/patinon/exploratory/abc/grammys:quarrel_parser",
        "//base_dir/patinon/exploratory/abc/grammys:salient_parser",
        "//base_dir/absl/flags:flag",
        "//base_dir/absl/flags:parse",
        "//base_dir/absl/memory",
        "//base_dir/absl/strings",
        "//base_dir/absl/strings:str_format",
        "//base_dir/java/antlr4:cc_runtime",
        # "//base_dir/java/antlr4/v4_7_1/Cpp/src:antlr4_runtime",
    ],
)
