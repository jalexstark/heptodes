---
author:
- J. Alex Stark
date: 2026
title: Jaywalk DAG Construction
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
<img src="figs-dagify/Dag-B.svg"
alt="An illustration of what we call a DAG segment, a selected portion of Jaywalk DAG that has no nodes to the left or right, but may have nodes to above and below, including above-left and so on. Let P and Q be two nodes within the segment with the largest and smallest converse tanks. From these we create a continuous border using the ANB and BNB chains from P and the ANA and BNA chains from Q. The chains intersect at the highest and lowest principal ranks. We call this border the eye of the segment. We call the parts of the chains that are within the segments the main chains, and we call the remainder of the chains beyond the intersections their continuations. Because there are no nodes to the left and right of the segment, the DAG itself provides two of the pairings. The ANA pairs are the rightmost children of each node’s connections, and the BNB pairs are the rightmost parents of a node’s connections." />
<figcaption aria-hidden="true">An illustration of what we call a DAG
<em>segment</em>, a selected portion of Jaywalk DAG that has no nodes to
the left or right, but may have nodes to above and below, including
above-left and so on. Let P and Q be two nodes within the segment with
the largest and smallest converse tanks. From these we create a
continuous border using the ANB and BNB chains from P and the ANA and
BNA chains from Q. The chains intersect at the highest and lowest
principal ranks. We call this border the <em>eye</em> of the segment. We
call the parts of the chains that are within the segments the
<em>main</em> chains, and we call the remainder of the chains beyond the
intersections their <em>continuations</em>. Because there are no nodes
to the left and right of the segment, the DAG itself provides two of the
pairings. The ANA pairs are the rightmost children of each node’s
connections, and the BNB pairs are the rightmost parents of a node’s
connections.</figcaption>
</figure>

<!-- Export at 80% -->

<figure>
<img src="figs-dagify/Dag-A.svg"
alt="An example merge that creates the DAG connections between left and right blocks. The process is much like that in a merge sort, intermeshing segments on the left and right. Segments with more than one node are shown with bounding boxes and eyes as in figure . A few nodes are highlighted by rendering with solid circles. Node P is a parent of Q, highlighting the fact that segment eyes do not isolate within a block. In contrast, it is never possible for a node within an eye on one side to have an edge connection to the other block. That is, all edges that cross the boundary between the blocks are between nodes on eyes. Nodes in an ANB chain on the left are parents to all the nodes in the main BNA chain of the next higher segment on the right. For example, A is a parent to B. But more than that, A is a parent to C on the continuation of that left BNA chain. Therefore continuation chains, at least on one side, have to be considered when constructing all the edges that join nodes between the blocks." />
<figcaption aria-hidden="true">An example merge that creates the DAG
connections between left and right blocks. The process is much like that
in a merge sort, intermeshing segments on the left and right. Segments
with more than one node are shown with bounding boxes and eyes as in
figure . A few nodes are highlighted by rendering with solid circles.
Node P is a parent of Q, highlighting the fact that segment eyes do not
isolate within a block. In contrast, it is never possible for a node
within an eye on one side to have an edge connection to the other block.
That is, all edges that cross the boundary between the blocks are
between nodes on eyes. Nodes in an ANB chain on the left are parents to
all the nodes in the main BNA chain of the next higher segment on the
right. For example, A is a parent to B. But more than that, A is a
parent to C on the continuation of that left BNA chain. Therefore
continuation chains, at least on one side, have to be considered when
constructing all the edges that join nodes between the
blocks.</figcaption>
</figure>

In between text.

<figure>
<img src="figs-dagify/Dag-C.svg"
alt="One approach to finding all additional edges in a Jaywalk DAG when combining a left block and a right block when each is a self-contained DAG. As was illustrated in figure , all new edges must be from the ANB of an eye on the left to a node in a BNA chain in an eye on the right. Therefore we can consider the segments one at a time on the left. In the manner of a merge sort we can focus on the next higher segment on the right, and start making connections there. Consider node P. Its current ANA pair is its current rightmost child. To this children C, B and A are added. This automatically updates its ANA pairing to A. The first new child for both Q and R is E. Likewise, the new children for S are E, D, C, B and A. Observe that the old ANA pair for S serves as the upper limit on the converse rank for its new children. The minimum set of children is the main BNA chain in the next right segment. Also note that the nodes that limit the range of child nodes themselves are members of an ANA chain, and are guaranteed to increase in converse rank. After processing this left segment, P is the rightmost parent of A, B and C and therefore the new BNB pairing for all three. Furthermore, we can consider the right segment “done” insofar as no further edges will be added to it. (This assumes that we process segments from bottom to top.)" />
<figcaption aria-hidden="true">One approach to finding all additional
edges in a Jaywalk DAG when combining a left block and a right block
when each is a self-contained DAG. As was illustrated in figure , all
new edges must be from the ANB of an eye on the left to a node in a BNA
chain in an eye on the right. Therefore we can consider the segments one
at a time on the left. In the manner of a merge sort we can focus on the
next higher segment on the right, and start making connections there.
Consider node P. Its current ANA pair is its current rightmost child. To
this children C, B and A are added. This automatically updates its ANA
pairing to A. The first new child for both Q and R is E. Likewise, the
new children for S are E, D, C, B and A. Observe that the old ANA pair
for S serves as the upper limit on the converse rank for its new
children. The minimum set of children is the main BNA chain in the next
right segment. Also note that the nodes that limit the range of child
nodes themselves are members of an ANA chain, and are guaranteed to
increase in converse rank. After processing this left segment, P is the
rightmost parent of A, B and C and therefore the new BNB pairing for all
three. Furthermore, we can consider the right segment “done” insofar as
no further edges will be added to it. (This assumes that we process
segments from bottom to top.)</figcaption>
</figure>

<figure>
<img src="figs-dagify/Dag-D.svg"
alt="Update to the ANB and BNA pairings when blocks are merged. In order to add the extra DAG edges, which is the main aim of such a merge, as illustrated in figure , we want the ANB and BNA chains, because these provide each node’s sequences of children and parents. If we need ANA or BNB pairs, these are available from the DAG. In other words, a side effect of adding edges is that ANB and BNA pairs are used to update ANA and BNB pairs. In contrast, as illustrated here, we use ANA and BNB chains to update the ANB and BNA pairs. This is performed, as we traverse segments, for the join between a segment on the right and the next immediately higher segment on the left. Example connections are shown. The node A currently has ANB pair x, and this needs to be updated to T. The node Q currently has BNA pair Y, and this needs to be updated to A. Hence the complete updates have two parts. We traverse the main ANA chain of the left segment, assigning the ANB pairs for A, B and C to T. Also we traverse the main BNB chain of the right segment, assigning the BNA pairs for P, Q, R, S and T to A." />
<figcaption aria-hidden="true">Update to the ANB and BNA pairings when
blocks are merged. In order to add the extra DAG edges, which is the
main aim of such a merge, as illustrated in figure , we want the ANB and
BNA chains, because these provide each node’s sequences of children and
parents. If we need ANA or BNB pairs, these are available from the DAG.
In other words, a side effect of adding edges is that ANB and BNA pairs
are used to update ANA and BNB pairs. In contrast, as illustrated here,
we use ANA and BNB chains to update the ANB and BNA pairs. This is
performed, as we traverse segments, for the join between a segment on
the right and the next immediately higher segment on the left. Example
connections are shown. The node A currently has ANB pair x, and this
needs to be updated to T. The node Q currently has BNA pair Y, and this
needs to be updated to A. Hence the complete updates have two parts. We
traverse the main ANA chain of the left segment, assigning the ANB pairs
for A, B and C to T. Also we traverse the main BNB chain of the right
segment, assigning the BNA pairs for P, Q, R, S and T to A.</figcaption>
</figure>
