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

# Section sample




<!-- \def\test#1#2{% -->
<!-- #2 $\to$ {\addfontfeature{#1} #2}\\} -->
<!-- <\!-- \fontspec{LinLibertine_R.otf} -\-> -->
<!-- \test{Ligatures=Historic}{strict Fluffy soufflé.} -->
<!-- \test{Ligatures=CommonOff}{firefly Fluffy soufflé.} -->

<!-- \fontspec{LinLibertine_R.otf} -->
<!-- \test{Ligatures=Historic}{strict Fluffy soufflé.} -->
<!-- \test{Ligatures=CommonOff}{firefly Fluffy soufflé.} -->



## Subsection sample

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



![Relationships between nodes in a dominance drawing.  From the
perspective of one node (at the origin here), nodes above and to the
right are descendants.  Nodes below and to the left are ancestors.
For Jaywalks we allow nodes to be exactly aligned vertically or
horizontally, and the relationship is ancestor to descendant.  All
other nodes are cousins, which means that the other node cannot be
reached via only forward edges or only backward
edges.\label{figG}.](figs-foundation/Builder-A-Relations.svg){width=250pt}

In between text.

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


![A Jaywalk that is a tree, displayed as a dominance drawing rotated
45-degrees clockwise.  The principal and converse ranks are shown, and
these are also the coordinates for the nodes before
rotation.\label{figB}](figs-foundation/Concepts-E.svg)


In between text.

![A Jaywalk that is longer and narrower than a tree.  The dominance
drawing is labelled and rotated as in figure \ref{figB}.  While a
Jaywalk like this is not in a clearly specific subcategory, it is
generally of the form that we might expect for states and state
transitions.  State Jaywalks may have multiple leaves like a tree but
are somewhat narrow.  We call these *chain-like* Jaywalks, and aim to
provide strong support for them from textual specification through to
rendering\label{figC}.](figs-foundation/Foundation-F.svg)


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
