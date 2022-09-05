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
title: Key D Records
author:
- J. Alex Stark
date: 2023
...

<!-- md-formatter on -->

# Purpose

INCLUDE-FILE: atoms/Salient-3854.md

## General purpose

# General KDRs

## Fact graphs

### Definitions

A *fact* is some characteristic or trait (not in Rust sense), especially that
notes an item of data's capabilities and eligibilities, or promises made about
it, especially restrictions.

### Key design

1.  Facts are often organized in DAGs. This allows for flexible combining along
    with partial ordering.

2.  Fact graphs are considered *alike* if they have common essence, that is have
    a common general purpose and obey the same rules as regards directedness.

3.  Facts are represented by vertices in a fact graph.

4.  Entities typically can have a set of facts applied to them not only from the
    same fact graph but also from fact graphs that are alike. This is a kind of
    sum, represented as a sum of vertices. This makes most sense for positive
    facts, that is capabilities.

5.  Entities can have sets of facts required of them from multiple fact graphs.
    They may also come from multiple orthogonal instances of the same fact
    graph, such as when than one structure has more than one member of the same
    type. In these cases the facts are combined as a kind of product.

### Consequences

*   The directed nature means that fact vertices are partially ordered, and this
    means that operations can be be applied to facts.
    *   The full set of facts selected by a specificed set of vertices are all
        those vertices reachable from them. If an entity possesses the facts
        specified by a vertex, it also possesses all descendant facts. When
        there are more facts, the full set is the union of descendant sets.
    *   The product of two sets is the intersection of the full sets.
    *   The complement of a set of facts can be found. Since there is partial
        ordering, it is perhaps more meaningful to handle

# Conceptual framework

The framework in which key decisions are made.

## Return for your effort

*   Patinon increases the demands on programmer to describe data. This is only
    acceptable if programmer is given much in return.

*   Guiding principle is to aid programmers in documenting "only once", with the
    understanding that content of, say, an early design document will be
    transferred into PQRS later.

*   Truths about data need to be presented as formatted clear documentation so
    that they need not be documented elsewhere.

*   Some control over what is presented, how, and in what order, must be
    provided so that auto documentation is ordered and less cluttered.

*   Not a full (Knuth-like) literate programming, but not forcing a style such
    as interface documentation, nor code organization such as directories and
    class-per-file.

*   Interface documentation is assumed.

# Jaywalks / Zebraix

## Coverage

## Unsorted

*   Graphs are used in a variety of ways.

*   Provide integrated support. Efficient and clear jaywalk syntax within
    Quarrel and Rostrum schema.

*   Integrated figure generation.

## KDRs

*   Use jaywalks for graph specification. Provide linearized representations.

## Rationale

*   Most graphs we need are planar or order-dimension 2.

*   Jaywalks incorporate drawing layout.

# Typestate, traits, capabilities, warrants

## Coverage

*   "States", which includes capabilities and restrictions.

*   Combination and connections between states.

## Capabilities DAG

### Definitions

Capabilities, which encompass much of Rust traits, control

*   the availability and

*   the applicability

of methods and functions, including accessors, both field and by-key (and
by-index).

### Key design

1.  Capabilities are added as leaf nodes of a DAG, and are defined as groups
    that are formed from graph vertices.

2.  Uses of capability graphs should normally be monotonic.

### Consequences

# Salient

For the purposes of discussion of Salient, comparisons are with wiki-like
markdown.

Everything refers to standard Patinon Quarrel-and-Salient-and-Rostrum (PQRS).
Extended versions might, for example, allow for html extensions.

## Definitions

*   *Annotation* is used to mean documentation associated with data entities and
    interfaces.

## Unsorted

*   Code, documentation and annotations need to be lightly and tightly
    integrated.

*   Markdown-like approach is familiar and efficient but somewhat broken.

*   Better provision for structured information. Smoother increase in complexity
    with customization. Better escape hatch, not resorting to html.

*   Restricted feature set. No PQRS provision for, say, resorting to HTML.
    Provision for externally generated tables, by file include, and SVGs as
    images.

*   More consistent syntax, simpler parsing.

*   Consistent handling of nesting and hierarchial document structure.

*   Less reliance on blank lines, which can then be used to indicate spacing
    intent such as list compactness.

*   Citations and footnotes, and the like, have core support. Less web-centric
    concept, more technical report.

*   Tables more fully-featured from the start.

# Rostrum

## Unsorted

*   Rostrum should be part of, or a subset of, Quarrel.

*   Rostrum should have a different data format and schema format.

*   Rostrum syntax should handle hierarchial data.

*   Contrary to traditional thinking, while Rostrum has much in common with JSON
    and protobuf text format, it is not so much a wire format. It is more for
    preset data, for configuration, for data initialization.

*   Support Salient escape-hatch data.

*   Support Quarrel basic structures.

*   Support jaywalks, including enumerations. Jaywalk definition syntaxes should
    be very much like Rostrum schema syntax.

*   Clearly distinguish set values, default values, field indices.

*   In data, avoid dependence on newlines, excepting where there are line
    comments or continuations without line termination.

*   In schema syntax, OK to rely on newline significance.

*   Support continuation forced line breaks, line join indicators, indentation
    alignment preference.

# Quarrel

*   Code syntax.

*   Code parser-formatter provided from outset in order to lighten syntax
    decisions. In other words, we can transform code into different syntax later
    if required.

*   Integrate with Salient, Rostrum and jaywalk "schema" syntax.

*   Comment positioning in 3 lisp-like locations. Comment positions are used to
    associate annotations.

*   OK to rely on newlines.

*   Invert annotation logic. Special characters or comment format indicate
    non-annotation. The default is for comments to be annotation rather than
    documentation.

*   Conditional compilation but not pre-processing.

# Fact graphs

Suppose that we implemented traits as a DAG where

*   the actual traits are in leaf nodes, and
*   internal nodes combine traits in groups.

For example, the trait for partical ordering requires inequality and equality,
whereas equality is a trait meaningful separately.

Traits on a structure would then be some combination of

*   traits on members, that is capabilities of member fields that we wish to
    expose, or
*   leaf traits, basically methods involving multiple fields, and
*   grouped leaf traits.

This concept is crucial. When we define a structure (type), its set of traits,
via accessors, expose a restricted interface to its members.

*   Typestates can have different traits sets for each state.

*   Containers can restrict, and perhaps even augment, the traits of its
    members. Much of the above is to create a more unified treatment of
    containers, such as a vector of SomeClass, and the treatment of classes,
    such as one with member of type SomeClass.

*   Trait DAGs have more capabilities as from leaves.

*   This establishes the ability to *AND* two sets of traits, in that only
    closest to leaves will satisfy both.

## Monotonicity

Directed typestate changes must result in monotonic changes in traits. This may
need to be bi-directional. Example: setting a member field. The subject of the
set needs at least a certain level of mutability. The object of teh set needs no
more than a certain level of mutability, so that it is guaranteed not to be
subject of unanticipated mutation.

Fact graphs are often linked. That is to say, the vertices in one graph may have
a one-to-one or one-to-many mapping to a second graph. There should be a
monotonically non-decreasing or non-increasing relationship. That is, if one
moves along an edge in the first graph, the mapped vertices in the second should
only move in a forward or reverse direction.

If application of a fact vertex implies aplication of all its descendants, or
alternatively ancestors, then monotonicity means that a move in the domain
vertex implies that the range vertices are subsetted, or supersetted, according
to directions.
