# diag-golden
[![Documentation](https://docs.rs/diag-golden/badge.svg)](https://docs.rs/diag-golden)
[![Latest Version](https://img.shields.io/crates/v/diag-golden.svg)](https://crates.io/crates/diag-golden)

**Diagramming golden tests in Rust.**

You can use diag-goldens to test the output of a parser, the order of
a graph traversal, the result of a simulation, or anything else that
should only change with human review.

## Usage

```rust
use diag-golden::Mint;
use std::io::Write;

let mut mint = Mint::new("tests/goldenfiles");
let mut file1 = mint.new_goldenfile("file1.txt").unwrap();
let mut file2 = mint.new_goldenfile("file2.txt").unwrap();

writeln!(file1, "Hello world!").unwrap();
writeln!(file2, "Foo bar!").unwrap();
```

When the `Mint` goes out of scope, it compares the contents of each file to its checked-in golden version and fails the test if they differ. To update the checked-in versions, run:
```sh
UPDATE_GOLDENFILES=1 cargo test
```

## Contributing

Pull requests are welcome!  This is part of a parent project called
*Zebraix*, which is in turn part of the *Heptodes* project.  There is
one github repository, with multiple crates.  For contribution
information, including community guidelines, see the [Heptodes
readme](https://github.com/jalexstark/heptodes).
