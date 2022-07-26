"""Build definitions."""

def simple_copy_file(name, src, target):
    input_path = "$(location %s)" % src
    output_path = "$(@D)/" + target

    native.genrule(
        name = name,
        srcs = [src],
        outs = [target],
        cmd = "cp %s %s" % (input_path, output_path),
    )

def gen_parser_lexer(
        name,
        src_file,
        src_dir,
        out_dir,
        test_type,
        out_extension = "",
        extra_args = []):
    """Pass a source file through the lexer, check that it can be reconstructed.

    Args:
      name: The name.
      src_file: The source filename. Leave empty for sample output.
      src_dir: The source directory.
      out_dir: The output directory.
      test_type: modifier string for names.
      out_extension: Output file extension.
      extra_args: Extra args.
    """
    src_full_name = src_dir + src_file
    srcs = [src_full_name]
    located_src_file = "$(location " + src_full_name + ")"

    if (test_type.startswith("debug_")):
        short_test_type = test_type[6:]
    else:
        short_test_type = test_type

    (out_name, dot, extension) = src_file.rpartition(".")
    if (out_extension == ""):
        out_extension = extension
    output_full_name = out_dir + out_name + "_" + short_test_type + dot + out_extension

    expected_gen_name = out_name + "_" + short_test_type + "_gen"
    if (expected_gen_name != name):
        fail(
            "The name \"" + expected_gen_name + "\" was expected for the test target name.",
            name,
        )

    native.genrule(
        name = name,
        srcs = srcs,
        outs = [
            output_full_name,
        ],
        cmd = " ".join([
            "$(location //base_dir/patinon/exploratory/abc:reform)",
            "--in_file=" + located_src_file,
            "--out_file=$(location " + output_full_name + ")",
            "--" + test_type,
        ] + extra_args),
        tools = [
            "//base_dir/patinon/exploratory/abc:reform",
        ],
        testonly = True,
    )

def test_parser_lexer(
        name,
        src_file,
        goldens_dir,
        out_dir,
        test_type,
        allow_updates,
        out_extension = ""):
    """Pass a source file through the lexer, check that it can be reconstructed.

    Use --test_arg="update_dir=$PWD" with local "test strategy" to update
    goldens instead of testing.

    Args:
      name: The name.
      src_file: The source filename. Leave empty for sample output.
      goldens_dir: The output directory.
      out_dir: The output directory.
      test_type: modifier string for names.
      allow_updates: True/false.
      out_extension: Output file extension.
    """
    src_full_name = out_dir + src_file
    srcs = [src_full_name]
    located_src_file = "$(location " + src_full_name + ")"

    if (test_type.startswith("debug_")):
        short_test_type = test_type[6:]
    else:
        short_test_type = test_type

    if (short_test_type == "lexer_pass_through"):
        golden_mod = ""
    else:
        golden_mod = "_" + short_test_type

    further_args = []
    if (allow_updates):
        further_args.append("--allow_updates=true")

    (out_name, dot, extension) = src_file.rpartition(".")
    if (out_extension == ""):
        out_extension = extension
    output_full_name = out_dir + out_name + "_" + short_test_type + dot + out_extension
    golden_full_name = goldens_dir + out_name + golden_mod + dot + out_extension

    expected_test_name = out_name + "_" + short_test_type + "_test"
    if (expected_test_name != name):
        fail(
            "The name \"" + expected_test_name + "\" was expected for the test target name.",
            name,
        )

    native.sh_test(
        name = name,
        srcs = ["//base_dir/patinon/exploratory/misc:diff_test.sh"],
        args = [
            "--golden_file=$(location " + golden_full_name + ")",
            "--out_file=$(location " + output_full_name + ")",
        ] + further_args,
        data = [
            golden_full_name,
            output_full_name,
        ],
        size = "small",
    )

def gentest_parser_lexer(
        name,
        aaa_gen_name,
        aaa_test_name,
        src_file,
        src_dir,
        out_dir,
        goldens_dir,
        test_type,
        out_extension = "",
        extra_args = []):
    """Pass a source file through the lexer, check that it can be reconstructed.

    Args:
      name: Macros are required to have a name name.
      aaa_gen_name: The name.
      aaa_test_name: The name.
      src_file: The source filename. Leave empty for sample output.
      src_dir: The source directory.
      out_dir: The output directory.
      goldens_dir: The output directory. IGNORED for pass-through tests.
      test_type: modifier string for names.
      out_extension: Output file extension.
      extra_args: Extra args.
    """

    allow_updates = True
    if (test_type == "debug_lexer_pass_through"):
        goldens_dir = src_dir
        allow_updates = False

    gen_parser_lexer(aaa_gen_name, src_file, src_dir, out_dir, test_type, out_extension, extra_args)
    test_parser_lexer(aaa_test_name, src_file, goldens_dir, out_dir, test_type, allow_updates, out_extension)
