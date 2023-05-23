--------------------------------------------------------------------------------

Heptodes documents and other content in `doc` directories are licensed under the
[Creative Commons Attribution 4.0 License](CC BY 4.0 license).

Source code licensed and code samples are licensed under the
[Apache 2.0 License].

The CC BY 4.0 license requires attribution. When samples, examples, figures,
tables, or other excerpts, are used in a tutorial, or a subdivision thereof, it
is sufficient to provide the complete source and license information once. This
must be close to the beginning, such as in an early acknowledgments slide. If
this is done, only short notes are required to be placed with each usage, such
as in figure captions.

[Creative Commons Attribution 4.0 License]: https://creativecommons.org/licenses/by/4.0/legalcode
[Apache 2.0 License]: https://www.apache.org/licenses/LICENSE-2.0

--------------------------------------------------------------------------------

<!-- md-formatter off (Document metadata) -->

---
title: The Essence of Rhodium
author:
- J. Alex Stark
date: 2023
...

<!-- md-formatter on -->

# Purpose

# Foundation

Rhodium was envisaged as bringing together a number of rather mundane use cases.
The foundational concept was that, by bringing together the requirements of
these uses, a language could inherently support critical practical needs. The
uses serve as specific target use cases and as exemplars. The hope and
expectation is that the practical utility of a language can be widenened
considerably by building in support for the underlying capabilities required by
these exemplars.

## Uses

Simple structures
:   Nestable collections of fields, few methods.

Hierarchical enumerations and jaywalks
:   Full reflection, optionally at runtime.

Configuration, general
:   Parameters for commissioning data, long function signatures.

Configuration, command-line
:   Entry (program) parameters, "getops", help.

Configuration, compilation
:   Conditional compilation, build and conditional code.

Configuration, tests
:   Test setup, test data.

Serialization and deserialization, text
:   Rhodium test format, JSON, ad-hoc.

Binary layouts, formats, protocols
:   Flatbuffer, protobuf.

Code generation
:   FFI support, compile-time reflection, ANTLR parsing.

Embedded data within documentation, code
:   Configuration, annotation, customization.

Self-documentating
:   Pretty-printing and drawing for schema and data.

Verification
:   Schema rules enforcement.

## Capabilities

Bridging adaptor DAGs; subsetting akin to contracts
:   Supports merging, compile configuration, easy structure subset declaration,
    code explorer aliasing (adaptor pass-through).

Reflection, optional at compile and runtime.
:   Name, short doc, long doc, getopt letter, getopt longname, type.

Value handling
:   offset-by-default for zero initialization - lossless minimal text
    representation - format string complile built-in const-expression
    calculation.

Multiple indexing
:   schema (declaration order, enum value, jaywalk principal, jaywalk obverse,
    wire field number.

Predicates and regularication
:   String regularization, range checks, URL validity.

Zebraix generation for rendering
:   Basically a Rhodium-data substructure, but tightly curated schema.

Seamless integration
:   Salient and Quarrel; embedding Rhodium data within schema.

## Miscellaneous notes

Versioning
:   Most applicable to wire-format capabilities.

Historical fields
:   Ability to withdraw, limiting write access.

Help verbosity
:   Support for main routes: verbosity levels, hierarchy, topics (such as
    sub-command).

Shared field definitions
:   In the extreme case, a dictionary-like set of field definitions with
    separate composition into actual structures.

# Syntax design

## Core considerations

Core syntax, schema and declaration forms.

*   Basic structure, nesting.
*   Key fields.
*   Jaywalk declaration.
*   Embedded Rhodium data, line comments as annotations.
*   Subsetting, recycling for declare-once, hierarchical subset.
*   Adaptor as variation on contracts.
*   Validation regularizers and predicates. Needs to be more than pure contract,
    such as range dependence on enum. This creates logical directedness.
*   Verbosity.

ALSO:

Flexible ability to associate enums with values in another enums and
auto-produce dreadful case statements.
