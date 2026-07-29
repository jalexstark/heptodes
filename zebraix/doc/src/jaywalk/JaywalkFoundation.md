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

<!-- mdformat off (Document metadata) -->

---
title: Jaywalk Foundation
author:
- J. Alex Stark
date: 2026
...

<!-- mdformat on -->

# Purpose

This document serves two purposes.  First, it defines Jaywalks, and
sets out the scope of what might be loosely called "The Jaywalks
Project".  Second, it sets out some technical details that are used by
more than one of the Jaywalk other technical documents.  As a whole,
this provides a foundation, though not an introduction or a user guide.

However, the result seems a bit messy.  While the mathematics that
underpins Jaywalks is well established, they do not quite fit into a
single category.  Moreover, we want *Jaywalk* to mean more than a
narrow definition, and to be associated with specific usage patterns
and tool capabilities. The technical exploration also seems messy.  We
attempt to justify the selection of topics, but it is a bit eclectic
and disjointed.

All of the above is like an apology in advance for what is perhaps
more of a hodgepodge than a coherent foundation.

<!-- \def\test#1#2{% -->
<!-- #2 $\to$ {\addfontfeature{#1} #2}\\} -->
<!-- <\!-- \fontspec{LinLibertine_R.otf} -\-> -->
<!-- \test{Ligatures=Historic}{strict Fluffy soufflé.} -->
<!-- \test{Ligatures=CommonOff}{firefly Fluffy soufflé.} -->

<!-- \fontspec{LinLibertine_R.otf} -->
<!-- \test{Ligatures=Historic}{strict Fluffy soufflé.} -->
<!-- \test{Ligatures=CommonOff}{firefly Fluffy soufflé.} -->

# Definitions

## Scope

We use the name *Jaywalk* to refer to (a) a particular class of
directed acyclic graph (DAG), along with (b) schemes for representing
them in code and in markdown-like text, (c) a set of manipulation
tools, and (d) ways of rendering them as drawings.  These aspects are
quite closely tied together.

## Core definitions

![An example Jaywalk shown as a dominance drawing.  It is perhaps
atypically complicated, but illustrates the main features of Jaywalk
DAGs.  For any two nodes, if one is above and to the right of the
other, it is a descendent reachable by forward edges.  If there is an
indirect path, that is via more than one edge in succession, then
there is no direct edge between them.  In a sense a Jaywalk DAG has
all necessary and no unnecessary edges.  This example has a global
sink but does not have a global source.  Many methods for manipulating
a Jaywalk are simpler if it has both, and the Jaywalk DAG is then an
*st-planar graph*.  In this example we could add a node at (0,0), and
we often do this, perform an analysis, and then trim the
result.\label{figA}](figs-foundation/Concepts-O-1.svg)

Jaywalks have multiple representations, but we choose one as
fundamental and use that as our reference (for data representation and
algorithms).  A Jaywalk is a set of nodes, each with two associated
integers: its principal rank and its converse rank.  The principal
ranks and the converse ranks serve as two sets each with distinct
integers.  The sets are often contiguous ranges, though the two ranges
may be different.  Thus the pairs (principal rank, converse rank) serve
as our reference representation.

For those who like a formal mathematical perspective, the nodes of a
Jaywalk form a partially ordered set (poset) of order dimension 2.
Comparison of principal ranks and of converse ranks is sufficient to
determine the order of any two nodes.  While we will not directly
refer to this perspective much, it is actually very important
(implicitly) when we consider ancestor-descendant relationships.  This
is in turn central to some practical uses of Jaywalks, such as
expressing typestate contracts or testing for compliance thereto.

Our secondary definition is via a similar representation with pairs of
real values rather than integers.  Each node as a coordinate pair
(principal value, converse value) that locates it in the plane.  An
example Jaywalk is illustrated in figure \ref{figA}.  We can take a
Jaywalk represented by rank pairs and convert to valid coordinates
simply by casting the integer ranks into real values.  We can convert
coordinates into ranks by sorting them.  That is, we sort the
principal values and converse values and use sort indices as ranks.
With Jaywalks we allow equal values using lexicographic sorting.  For
instance, when sorting nodes by principal value (x coordinate), if two
values are equal, comparison of the obverse values resolves the tie.

## Jaywalk DAGs and dominance drawings

![Relationships between nodes in a dominance drawing.  From the
perspective of one node (at the origin, shown here as a solid dot),
nodes above and to the right are descendants.  Nodes below and to the
left are ancestors.  For Jaywalks we allow nodes to be exactly aligned
vertically or horizontally, and the relationship is ancestor to
descendant.  All other nodes are cousins, which means that the other
node cannot be reached via only forward edges or only backward
edges.\label{figG}.](figs-foundation/Builder-A-Relations.svg){width=250pt}

Every Jaywalk is associated with a DAG, so closely that the DAG is
often thought of as "the Jaywalk".  Nevertheless, we use rank pairs as
the fundamental definition and data representation.  (This is in large
part because it is easier to work with rank pairs.)  A Jaywalk's DAG
is relatively simple to define in three rules.  It can be helpful to
draw the DAG, as in figure \ref{figA}.  The first rule is that, for
any two nodes one is the ancestor of the other if and only if it has
both lesser principal rank and lesser converse rank.  This test is
illustrated in figure \ref{figG}.  The second rule is that, if the
descendant can be reached from the ancestor (wholly in the same
direction) indirectly through a third node, we do not add an edge
directly linking the two.  Another way to apply this rule is to start
with all edges and delete direct edges in favour of indirect paths.
That is known as *transitive reduction*.

A DAG reconstruction like this, drawn pictorially, is known as a
*dominance drawing*.  (Strictly speaking a dominance drawing is
constructed from coordinate value pairs rather than ranks.)  The third
rule is that children and parents are ordered as they are laid out in
the dominance drawing.  We adopt as our convention the use of
clockwise order.  From a parent's perspective its children are ordered
from the left to right.  This means that children are ordered from the
lowest principal rank to the highest, and the highest converse rank to
the lowest.  From a child's perspective, parents are also ordered left
to right, and therefore from highest principal rank to lowest.  (This
convention means that if nodes are written out in text in order of
principal rank the text order matches the visual order.  The ordering
of parents is more of an arbitrary choice, and it means that if a
dominance drawing is rotated 180 degrees the orders are preserved.)

# More depth

The rank-pair representation of Jaywalks can be readily converted to a
set of coordinate pairs.  We choose to use lexicographic sorting for
the reverse process.  However, that depends on numerical (floating
point) equality comparison, and so we prefer only to use if for
coordinate pairs that are known to have no equal values.  For
practical uses of Jaywalks we expect to work primarily with rank pairs
and choose coordinates based on those.  We also discussed how DAG
edges can be found from ranks and a Jaywalk DAG rendered with nodes at
their coordinates is called a dominance drawing.  The described
process of constructing a DAG from ranks serves as a definition, but
is insufficient as an algorithm.  We later describe a tool that is
useful in building an efficient algorithm, but leave the algorithm
itself for a separate document.  Next here we will fill in another
missing piece, namely the task of finding the ranks from  a DAG.

## From DAG to ranks

Finding the ranks for a suitable DAG is quite straightforward.  We
must note that this is not a process that would be applied
automatically.  We claimed previously that DAGs are not an ideal
internal representation.  DAGs are more general than Jaywalks, and in
particular the nodes of a DAG may not constitute a poset of order
dimension 2.  In other words, DAGs are not in general valid Jaywalks.
Furthermore, the simple method relies on a suitable order of child (or
parent) edges.  Nonetheless, the method is useful for manual Jaywalk
creation.  If Jaywalks catch on, then no doubt user-friendly tools
will be developed that make Jaywalk creation easier.  In the meantime,
the following method generally suffices.

![Two rank sequences for the Jaywalk DAG of figure \ref{figA}, shown
on trees with the edges traversed in the DFSs.  The left diagram shows
the principal ranks.  These are found via a DFS topological sort,
traversing children right to left, and numbering children first in
descending rank.  For the converse rank children are traversed left to
right.  These are shown in the right diagram.  A global source is
added in order to illustrate how it simplifies the handling of
multiple sources.  This requires knowing their order.  The global
source would be dropped after ranks are
obtained.\label{figE}](figs-foundation/NetTreesDfs.svg)

The method is to use two topological sorts of the DAG using
depth-first search (DFS).  The two sorts for the DAG in figure
\ref{figA} are shown in figure \ref{figE}.  Nodes are ranked in
reverse (from high to low) with parents ranked after all children are
ranked, ensuring the rule ancestors have lower ranks.  For principal
ranks, children are visited right to left, effectively in reverse rank
order.  Therefore the right side of the graph has the higher principal
ranks.  Conversely, the converse ranks are found using a DFS
topological sort visiting children left to right.  The lower part of
that graph has the lower ranks.

If the graph has multiple roots, these are traversed in like manner.
An equivalent approach is to add a global root, perform the
topological sorts, and then discard the extrapolated node.  This is a
trick often used in implementations, as it ensures uniform invariants
in data structures during manipulation.

## Jaywalk details


![A Jaywalk that is a tree, displayed as a dominance drawing rotated
45-degrees clockwise.  The principal and converse ranks are shown, and
these are also the coordinates for the nodes before
rotation.\label{figB}](figs-foundation/Concepts-E.svg)

![A Jaywalk that is longer and narrower than a tree.  The dominance
drawing is labelled and rotated as in figure \ref{figB}.  While a
Jaywalk like this is not in a clearly specific subcategory, it is
generally of the form that we might expect for states and state
transitions.  State Jaywalks may have multiple leaves like a tree but
are somewhat narrow.  We call these *chain-like* Jaywalks, and aim to
provide strong support for them from textual specification through to
rendering\label{figC}.](figs-foundation/Foundation-F.svg)


In addition to figure \ref{figA}, two Jaywalks with characteristic
forms are shown in figure \ref{figB} and figure \ref{figC}.  The
dominance drawings are rotated 45-degrees clockwise to give them a
more familiar form.  A specific form is the tree (figure \ref{figB}),
with no converging branches.  These are typcially wide an shallow.
This form represents hierarchies of enumerations and other nested
classifications.  The example shown in figure \ref{figC} is not an
exact category.  Rather it is chosen to represent Jaywalks that are
broadly longer and narrower.  We expect many state applications to be
like this, with a few branches that may converge and a general
"chain-like" structure.  We attempt to provide specific support for
tree-like Jaywalks and chain-like Jaywalks in order to maximize
readability and efficiency.

![When used to describe states, it is often useful for Jaywalks to
have edges directly linking states that would be removed by transitive
reduction.  Such scenarios are handled by adding a *waypoint* node,
shown here smaller and shaded.  State transitions would not stop in
this state, but would pass through it, creating an extra transition
from A to D.  When Jaywalks are used as states, the system is may be
between states rather than at one.  Or a system may be in a range of
states.  The waypoint has the important feature that we can
distinguish which edges we are "on" when in a transitory pseudo-state
between A and D.\label{figH}](figs-foundation/Concepts-H.svg)

When Jaywalks are used to codify DAGs of states, the transitive
reduction rule is potentially inconvenient.  An pattern is illustrated
in figure \ref{figH}, in which there is a transition path from A to D
via I, and so the Jaywalk DAG will not have a direct transition from A
to D.  The technique with Jaywalks is to insert a *waypoint* node
(state) and introduce a path from A to D via W.  This technique has at
least two advantages.  One is that it reduces crossings in the
drawing.  The other is that the waypoint provides ways to distinguish
which path the state is "on" so-to-speak.  When handling, say,
typestate there are often scenarios in which the system is not
necessarily in one distinct state, and can indeed be in transition
between states.  For these reasons the Jaywalk tools provide specific
support of waypoints, not least in how they are rendered in drawings.

## Further notes on purpose

The original motivation for Jaywalks was the desire to test simple
DAGs of typestate and other state that unfolds during code execution.
Posets of order dimension 2 enable specification and testing of
contracts using two straightforward (rank) comparisons.  There may be
cases where order dimension of 3 or more might be required, but common
cases can be handled by standard Jaywalks.  It may even prove
worthwhile marking Jaywalks that are trees.  The second motivation
behind Jaywalks followed with the realization that their DAGs have a
standard drawing, and one that is controlled by the Jaywalk
specification.  Therefore Jaywalks can automatically be rendered for
visualization.  It is very important for computer languages, such as
those that follow up on and build upon Rust, to give back to the
programmer.  Clean automatic documentation will be a central way of
rewarding the effort incurred in defining and satisfying typestate and
contracts.  The third motivation (chronologically, again) was that
Jaywalks, if defined as posets as rank pairs, can be given clean
representation in text and in data structure code.  Furthermore the
data structures can be manipulated quite readily.

We expect Jaywalks that are used in reality to be fairly small and
fairly simple.  The analysis herein, and in accompanying docs, and in
code, handles complicated Jaywalks.  This is so that the tools scale
well, and so that we understand Jaywalks sufficiently comprehensively.

# Banba forms

We now introduce a set of four related Jaywalk representations that we
use as the basis for our textual representations, and that is key to
developing our method of constructing DAGs from ranks. These two tasks
are very different, and their specifics are discussed elsewhere.

## Factorially paired

Consider the rank-pair representtation of a Jaywalk of size $N$.  For
simplicity, let both the principal and converse ranks be integers in
the range $[0,N)$.  If we list the nodes in order of principal ranks
we only need state the converse ranks.  The list is effectively $1$ of
$N!$ orderings of $N$ integers.  So this is the number of unique
Jaywalks of size $N$.  However, this is not a convenient
representation for humans, for two reasons.  First, we want to refer
to enumerated values and states (that is, Jaywalk nodes) by name
rather than by number.  Second, validating a Jaywalk is a visually
(human) and an automatically awkward process.  Valid Jaywalks have no
duplicate converse ranks, and checking for duplicates, while not
complicated, is at least conceptually awkward.  While we do not
discuss textual encodings in any detail here, we note that we want
something like "(green, amber)" pairs in, say, order of principal
rank.  In general we prefer not to have variable-length lists of
associations.  One could list each node's children in the DAG, but we
encounter the problem that not all DAGs are Jaywalks, and verifying
DAGs is expensive.  Furthermore variable-length encodings are not
easily readable.  We should mention that it is helpful to avoid
explicit rank numbering because we might add or remove a node in the
middle and renumbering would result in obscuring version diffs.

All of this is to say, in motivation, that we want something like
"(green, amber)" symbolic pairings, say for states of traffic lights.
We can list in the order of principal rank of the first state (green),
and discover the converse ranks from the pairings.

## BNB form

We develop the idea of a paired expression in 3 steps.  First, as we
note above, if we list the nodes in order of principal rank, then any
pairing scheme need only enable us to ascertain the converse order.
Second, consider trees that we already described.  In particular,
consider the tree on the right in figure \ref{figE}.  Suppose that we
encodes this tree by pairing each node with its parent.  (Each edge in
the tree, in reverse direction, indicates the pairs.)  There are $N!$
possible pairings, because each node either has no parent or is paired
with a node of lower principal rank.  This is a valid set of pairs.
Third, consider each node in isolation.  It is paired with the node
that, among those with lower principal rank, has the highest rank.  We
call this the BNB scheme, because among all nodes Before it in
principal rank, its pair is the Next node just Below in converse rank.

We demonstrate that this pairing scheme works because there is a
simple algorithm for constructing a list of nodes in ascending order
of converse rank.  Begin with an empty list and visit nodes in
ascending order of principal rank.  Insert each node just after the
node with which it is paired.  This shows also that every member of
the size-$N!$ valid pairings maps 1:1 to the set of Jaywalks.  We may
have multiple global sources in a Jaywalk and these become multiple
tree roots in the DFS tree and hence are unpaired.  Since we traverse
these in increasing principal rank, they must have decreasing converse
ranks.  We simply insert them at the beginning of the list.

The method can be used in reverse, that is to find BNB pairings
directly from rank pairs.  A bit of data structure work is required.
First create linkage between nodes in order of converse rank.
Traverse the nodes in reverse order of principal rank, deleting from
the linked list and noting the previous node in the linkage as the
node's pair.  If the node is at the beginning of the list when
deleted, it is a global root.

## Banba variations

![Four variations on node pairings that, chained together, form trees.
These were generated for the Jaywalk DAG in figure \ref{figA}.
Clockwise from top-left these are BNA, ANA, ANB and BNB, where
B=before and A=after.  The first letter refers to the principal rank
(x-coordinate) and the second refers to the converse rank
(y-coordinate).  For example, in the BNA pairings, each node is paired
with a node whos principal rank comes before and whose converse rank
comes after.  Among all such nodes the one with the least converse
rank is selected, that is the Next After.  In other words, for each
node find the next node above that is to the left.  Nodes on the
perimeter of the figure have no nodes to pair, and so these become
roots of trees.  These pairings are used as the basis for text
(in-code) representations of Jaywalks.  They are also used in
algorithms for the construction of Jaywalk DAGs, that is finding the
ordered parent-child edges from the node ranks.  These tasks are
discussed in accompanying documents.  Observe that the ANA parigins
are the rightmost child of each node and that the BNB pairings are the
rightmost parents (as viewed from the node towards the bottom-left).
The BNB tree is the same as the DFS tree for converse ranks in figure
\ref{figE}.  The ANA tree is also a topological DFS, but the ranks are
reversed and the search begun from the top-left.  Any of these
pairwise associations is sufficient to encode a Jaywalk, as explained
in the main text by means of a reconstruction
method.\label{figF}](figs-foundation/NetTreesAb.svg)

The scheme described above we call the BNB pairing.  There are four
variations of this, the others called BNA, ANA and ANB.  The first
letter refers to the principal ranks, before or after.  The last
letter refers to converse ranks.  For example, in the ANB scheme each
node is paired with a node with higher (after) principal rank and the
next-before (highest among those lower) converse rank.  The four
schemes are illustrated in figure \ref{figF}.  If a Jaywalk is
represented as pairs using any of these schemes, the converse ranks
can be found in $\mathcal{O}(N)$ work using the same method as for BNB
pairings, with minor variations.

# The scope of the Jaywalks project

![The scope of the Jaywalks tooling, illustrated as a progression that
encapsulates the transformations, analysis and rendering that we
expect to be most common.  Most usages will begin with a textual
(code) representation and be parsed and converted to rank pairs or
stored in a data structure as pairs.  One advantage of the textual
representations (as explored in detail in an accompanying document) is
that they also encode the ordering of nodes by converse rank.  One of
the biggest technical challenges is the mathematical and algorithmic
task of constructing a Jaywalk's DAG from its ranks.  (This is
explored in another accompanying document.)  The reverse process can
use DFS topological sorting, and that is relatively simple.  Rendering
Jaywalk DAGs as dominance drawings is algorithmically simple.  The
drawings are convenient, but their layout is often not ideal.
Therefore the Jaywalk ecosystem will include tools for rendering in a
set of polished layouts and
styles.\label{figD}](figs-foundation/Foundation-D.svg)

Jaywalks are expressed in a variety of ways.  Some of these we have
mentioned briefly, and others we have explore in more depth.  The
remaining details are set out in accompanying documents.  One can view
these expressions in a sequence, as shown in figure \ref{figD}.  On
the one end is a textual representation that is used in code or within
markdown-like documentation.  On the other end is a polished drawing
that renders the Jaywalk in a clean, easily readable form that is well
laid out.  In between are a sequence of transformations.  The textual
representation is parsed and used to create the two ranks for each
node.  This is our fundamental expression, because it encodes a
Jaywalk in a necessary, sufficient and convenient form.  We can create
a DAG for the Jaywalk.  The combination of rank pairs (for node
locations) and DAG edges (for connectivity) can be used to create a
dominance drawing.  These are reasonably clear, but nonetheless it is
preferrable to fine-tune the dominance drawing according to the use
case before presenting to users.

When Jaywalks are used in code or markdown documents, the Jaywalk tool
set automatically creates polished drawings from text Jaywalks.  For a
lot of scenarios rank pairs are sufficient.  DAGs are needed when, for
example, testing specific direct state transitions, but rank pairs are
enough for testing if a transitions path exists.
