# Zebraix

Zebraix is a set of utilities for handling graphs that can be described as
partially ordered sets with order-dimension 2.

## Disclaimer

This project is not an official Google project. It is not supported by Google
and Google specifically disclaims all warranties as to its quality,
merchantability, or fitness for a particular purpose.

## Ephemeral notes

### Create and module organization

Zebraix will likely end up with crates z-graph, z-test-utils, z-spline, j-graph.
The test utils may pull in lots of things that other crates do not need outside
of testing.
