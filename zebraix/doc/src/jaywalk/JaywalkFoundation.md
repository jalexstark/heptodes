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

In between text.


![Fig B. A tree with principal and obverse indices (shown as pairs) that are found by
depth-first search.\label{figB}](figs-foundation/Concepts-E.svg)


In between text.

<!-- mdformat off (Document metadata) -->



![Fig A. Example of a jaywalk graph with global source and sink, which makes it an
st-planar graph. Jaywalks are DAGs that are transitively irreducible, and have
external sources and sinks. The graph is shown as a dominance drawing with
coordinates that correspond to principal and obverse orderings. All jaywalks are
posets of order-dimension 2, so the orderings are sufficient for a
jaywalk.\label{figO1}](figs-foundation/Concepts-O-1.svg)



![Fig H. Part of a graph where a *waypoint* has been added along the direct path
between A and D so that the path is not removed by transitive reduction. This
has the important result that we can distinguish which edges we "are on" when in
a transitionary pseudo-state between A and D. ](figs-foundation/Concepts-H.svg)

![Fig C.  Within a graph, a subgraph can be defined as all nodes that
are descendents of an ancestor vertex and also ancestors of a
descendent vertex. Here the ancestor is A and the descendent
D.](figs-foundation/Foundation-F.svg)


![Fig E.\label{figE}](figs-foundation/NetTreesDfs.svg)

![Fig F.\label{figF}](figs-foundation/NetTreesAb.svg)


![Fig G. Lexicographic comparison of principal and obverse indices. The diagram shows,
with respect to the vertex in the centre, the locations or combinations of
indices that a vertex can have to be in a dominant or co-dominant relationship.
If a vertex has strictly greater principal and obverse indices (upper-right
quadrant) it is reachable in the dominance drawing. Likewise, vertices in the
bottom-left quadrant can reach the vertex in question. In contrast, vertices in
the top-left and bottom-right quadrants have a reachability relationship only in
the co-dominance drawing. On the axes, we employ a lexicographic comparison so
here vertices are reachable only in the dominance. Therefore dominance and
co-dominance drawings are not quite complementary in regard to handling
comparisons for reachability\label{figG}.](figs-foundation/Builder-A-Relations.svg)

![Fig D.\label{figD}](figs-foundation/Foundation-D.svg)
