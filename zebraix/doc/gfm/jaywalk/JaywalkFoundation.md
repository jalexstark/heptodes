---
author:
- J. Alex Stark
date: 2026
title: Jaywalk Foundation
---

<!--%% Copyright 2025 Google LLC
%%
%% Licensed under the Apache License, Version 2.0 (the "License");
%% you may not use this file except in compliance with the License.
%% You may obtain a copy of the License at
%%
%%      http://www.apache.org/licenses/LICENSE-2.0
%%
%% Unless required by applicable law or agreed to in writing, software
%% distributed under the License is distributed on an "AS IS" BASIS,
%% WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
%% See the License for the specific language governing permissions and
%% limitations under the License.
%%
&#10;<!-- Copy from pdflatex header. -->

------------------------------------------------------------------------

Heptodes documents and other content in `doc` directories are licensed
under the [Creative Commons Attribution 4.0
License](CC%20BY%204.0%20license).

Source code licensed and code samples are licensed under the [Apache 2.0
License](https://www.apache.org/licenses/LICENSE-2.0).

The CC BY 4.0 license requires attribution. When samples, examples,
figures, tables, or other excerpts, are used in a tutorial, or a
subdivision thereof, it is sufficient to provide the complete source and
license information once. This must be close to the beginning, such as
in an early acknowledgments slide. If this is done, only short notes are
required to be placed with each usage, such as in figure captions.

------------------------------------------------------------------------

<!-- mdformat off (Document metadata) -->
<!-- mdformat on -->

In between text.

<figure>
<img src="figs-foundation/Concepts-E.svg"
alt="Fig B. A tree with principal and obverse indices (shown as pairs) that are found by depth-first search." />
<figcaption aria-hidden="true">Fig B. A tree with principal and obverse
indices (shown as pairs) that are found by depth-first
search.</figcaption>
</figure>

In between text.

<!-- mdformat off (Document metadata) -->

<figure>
<img src="figs-foundation/Concepts-O-1.svg"
alt="Fig A. Example of a jaywalk graph with global source and sink, which makes it an st-planar graph. Jaywalks are DAGs that are transitively irreducible, and have external sources and sinks. The graph is shown as a dominance drawing with coordinates that correspond to principal and obverse orderings. All jaywalks are posets of order-dimension 2, so the orderings are sufficient for a jaywalk." />
<figcaption aria-hidden="true">Fig A. Example of a jaywalk graph with
global source and sink, which makes it an st-planar graph. Jaywalks are
DAGs that are transitively irreducible, and have external sources and
sinks. The graph is shown as a dominance drawing with coordinates that
correspond to principal and obverse orderings. All jaywalks are posets
of order-dimension 2, so the orderings are sufficient for a
jaywalk.</figcaption>
</figure>

<figure>
<img src="figs-foundation/Concepts-H.svg"
alt="Fig H. Part of a graph where a waypoint has been added along the direct path between A and D so that the path is not removed by transitive reduction. This has the important result that we can distinguish which edges we “are on” when in a transitionary pseudo-state between A and D." />
<figcaption aria-hidden="true">Fig H. Part of a graph where a
<em>waypoint</em> has been added along the direct path between A and D
so that the path is not removed by transitive reduction. This has the
important result that we can distinguish which edges we “are on” when in
a transitionary pseudo-state between A and D.</figcaption>
</figure>

<figure>
<img src="figs-foundation/Foundation-F.svg"
alt="Fig C. Within a graph, a subgraph can be defined as all nodes that are descendents of an ancestor vertex and also ancestors of a descendent vertex. Here the ancestor is A and the descendent D." />
<figcaption aria-hidden="true">Fig C. Within a graph, a subgraph can be
defined as all nodes that are descendents of an ancestor vertex and also
ancestors of a descendent vertex. Here the ancestor is A and the
descendent D.</figcaption>
</figure>

<figure>
<img src="figs-foundation/NetTreesDfs.svg" alt="Fig E." />
<figcaption aria-hidden="true">Fig E.</figcaption>
</figure>

<figure>
<img src="figs-foundation/NetTreesAb.svg" alt="Fig F." />
<figcaption aria-hidden="true">Fig F.</figcaption>
</figure>

<figure>
<img src="figs-foundation/Builder-A-Relations.svg"
alt="Fig G. Lexicographic comparison of principal and obverse indices. The diagram shows, with respect to the vertex in the centre, the locations or combinations of indices that a vertex can have to be in a dominant or co-dominant relationship. If a vertex has strictly greater principal and obverse indices (upper-right quadrant) it is reachable in the dominance drawing. Likewise, vertices in the bottom-left quadrant can reach the vertex in question. In contrast, vertices in the top-left and bottom-right quadrants have a reachability relationship only in the co-dominance drawing. On the axes, we employ a lexicographic comparison so here vertices are reachable only in the dominance. Therefore dominance and co-dominance drawings are not quite complementary in regard to handling comparisons for reachability." />
<figcaption aria-hidden="true">Fig G. Lexicographic comparison of
principal and obverse indices. The diagram shows, with respect to the
vertex in the centre, the locations or combinations of indices that a
vertex can have to be in a dominant or co-dominant relationship. If a
vertex has strictly greater principal and obverse indices (upper-right
quadrant) it is reachable in the dominance drawing. Likewise, vertices
in the bottom-left quadrant can reach the vertex in question. In
contrast, vertices in the top-left and bottom-right quadrants have a
reachability relationship only in the co-dominance drawing. On the axes,
we employ a lexicographic comparison so here vertices are reachable only
in the dominance. Therefore dominance and co-dominance drawings are not
quite complementary in regard to handling comparisons for
reachability.</figcaption>
</figure>

<figure>
<img src="figs-foundation/Foundation-D.svg" alt="Fig D." />
<figcaption aria-hidden="true">Fig D.</figcaption>
</figure>
