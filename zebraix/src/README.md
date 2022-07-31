# Zebraix main code directory

This contains, for now, one codebase for Zebraix.  In the future we are likely
to divide out the minimal version from the expanded version.

## Running Rust

Zebraix and Git work perhaps a little more smoothly with a temporary directory
for the Cargo target directory.  This is

```sh
export CARGO_TARGET_DIR="/tmp/cargo-target"
cargo clean
cargo test -p test-samples
```

To regenerate test golden files, set an environment variable.

```sh
env REGENERATE_GOLDENFILES=1 cargo test -p test-samples
```

## Formatting code

If editing in emacs, one can set customization for format-on-save.

```lisp
(custom-set-variables
 ;; custom-set-variables was added by Custom.
 ;; ...
 '(rust-format-on-save t)
 '(rust-rustfmt-bin "~/bin/rustfmt.sh")
 ;; ...
 )
```

Then the formatting command can be a shell script if that is more convenient.
The Patinon group of projects set their own formatting rules.

```sh
#!/bin/sh

/path/to/installed/rustfmt --config use_small_heuristics="Max",newline_style="Unix" "$@"
```
