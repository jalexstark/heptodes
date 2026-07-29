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
title: Jaywalk DAG Construction
author:
- J. Alex Stark
date: 2026
...

<!-- mdformat on -->


# Scope

In this document we set out some methods for constructing a Jaywalk's
DAG from its rank pairs.  We assume thorough familiarity with all
material in the [Jaywalk Foundation document](JaywalkFoundation),
and we will repeat very little here.  Rather, the sections that follow
should be treated as if they were appended to that document.  Let us
specify the scope of our task.  Our core algorithm takes a Jaywalk
represented as a set of rank pairs sorted by principal rank.  and
creates the DAG.  The output is a set of nodes, each with ordered
lists of its children and parents.  A list of source roots is
produced.  A typical realization of the algorithm would consume and
produce node data in vectors indexed on the principal rank.  In many
use cases the sort order of the converse ranks is known.  We assume
that when available this would enable lookup of the principal rank
from the converse rank in $\mathcal{O}(1)$ time.  Our core algorithm
does not use this information.  However, a more elaborate algorithm
can exploit it in order to reduce the theoretical
(big-$\mathcal{O}$) performance and the practical performance on
smaller Jaywalks.

Recall one decision from the foundations document.  When we construct
lists of children we do so clockwise from lowest principal rank to
highest, and therefore highest converse rank to lowest.  Lists of
parents are likewise clockwise (also left to right from the node's
perspective), that is highest principal rank to lowest and lowest
converse rank to highest.  We can rotate the DAG 180 degrees by
exchanging lists of parents and children.  Finally we can apply the
algorithms to (principal value, converse value) coordinate pairs
instead of to rank pairs.  We note the handling of equal values in
various places.


# Essentials

## Simple approach

Many, if not all, deeply optimized sort library functions use
"inefficient" methods such as insert sort for small datasets and for
the first stages of larger sorts.  This is because bookkeeping and
branching can be expensive.  For Jaywalk DAG construction it makes
sense to begin with a simple method, albeit one with poor asymptotic
performance.

Our simple method directly searches for all children of every node.
Traverse all nodes with greater principal rank in increasing order of
principal rank.  We skip over nodes with lower converse rank.  The
first with greater converse rank is the first child, and we note its
converse rank.  Subsequent valid nodes are children if they have lower
converse rank than the previous child.  (When applying to coordinate
values rather than to ranks, we use lexicographic sorting of principal
values over converse values, and use greater than or equal, instead of
greater than.)  Lists of parents can be constructed in a second phase,
or a node can be added as a parent to each child node as it is found
in the main traversal.  Parents are inserted at the beginning of
parent lists.

## Complete complexity

![A dominance drawing of a Jaywalk and its corresponding co-dominance
drawing.  A co-dominance drawing can be constructed by negating either
the principal or converse values, constructing a dominance drawing,
and flipping the result to restore the node
locations\label{figP}.](figs-dagify/Builder-B.svg)


At this point we need to address algorithmic complexity.  First we
introduce a variation on the Jaywalk DAG.  As discussed in the
[Jaywalk Foundation document](JaywalkFoundation), the dominance
drawing is a Jaywalk DAG drawn out in a specific arrangement.  An
example Jaywalk is rendered in this way in figure \ref{figP}.  If we
exchange the principal and converse values most of the connectivity is
preserved.  However, if we apply a reflection to either set of values,
such as negating the converse values, the dominance drawing can change
dramatically.  The results, if the drawing is flipped back to the
original nodes locations is called the codominance drawing.  Figure
\ref{figP} also shows the matching codominance drawing for its
Jaywalk.


![A simple dominance graph can have a complex corresponding
co-dominance graph, and vice versa. The worst-case complexity is a
graph with the number of edges proportional to the square of the
number of vertices.  This has implications for some construction
algorithms.  Specifically, if an algorithm implicitly traverses
co-dominance edges, its complexity is at least $\mathcal{O}(N^2)$.
This applies even if the dominance drawing (and Jaywalk DAG) has few
edges, such as $\mathcal{O}(N)$ as
illustrated\label{figQ}.](figs-dagify/Concepts-Q.svg)

One issue is that the complexity of a codominance drawing can be very
different from its dominance drawing, as illustrated in figure
\ref{figQ}.  Let $N$ be the number of nodes, and $E$ the number of
edges in the Jaywalk DAG.  Let $C$ be the number of edges in the
codominance drawing.  The example shown in figure \ref{figQ} is close
to the extreme, that is $$E\approx N$$ $$C\approx \frac{N^2}{4}.$$ If
an algorithm requires $\mathcal{O}(C)$ bookkeeping, then in the worst
case the algorithm's complexity would be $\mathcal{O}(N{}^2)$.  (This
assumes no other scaling problems.)

## Minimum complexity

We use the term *converse sort* to refer to additional information
that encodes the ordering of nodes by converse rank.  Suppose that the
basic input data is a vector of node data, ordered by principal rank.
Suppose that each record contains at least the node's principal and
converse ranks, and the index back to its vector (sort by principal
rank) location.  The converse sort might be represented as previous
and next links, with the index to the node with the lowest converse
rank serving as the beginning of the linked list of nodes in order or
converse rank.

Our core algorithm does not "know" the converse sort.  That is to say,
it does not make use of that information even if it is available at
the input.  (Note that this is for the core algorithm.)  Once the DAG
is constructed, we can find such as sorting by depth-first search.  In
other words, the DAG contains the sort.  Therefore constructing a DAG
must involve $\mathcal{O}(N\log(N))$ work.  It must also involve
$\mathcal{O}(E)$ work, not least in performing the bookkeeping
required to create the output.  Our strategy for the Jaywalk project
has been to develop a core algorithm with a combination of
$\mathcal{O}(N\log(N))$ and $\mathcal{O}(E)$ work. The we can
accelerate this if a converse sort is available.

If one knew in advance that a Jaywalk were a tree, $E\propto N$ and
the DAG construction work should be $\mathcal{O}(N)$.  The question is
naturally whether an algorithm can be $\mathcal{O}(N\log(N))$ plus
$\mathcal{O}(E)$, but guarantee to be $\mathcal{O}(N)$ in such a
special case.  This we will explore later.  In practical reality a
more pertinent question is whether the overall method, which may
utilize a combination of strategies, is fast for small Jaywalks.  For
most uses that we envisage, Jaywalks will be small ($N<20$, say).  The
fact that many will be trees may well prove moot.

# Non-universal construction

## Trees construction

If it is known ahead of time that a Jaywalk is a tree, and the
converse sort is known, then a fast method can be used.  In the
[Jaywalk Foundation document](JaywalkFoundation) the basis of such a
method was described as finding BNB pairings from rank pairs.  Or put
another way, the BNB pairings provide the complete set of child-parent
associations for a tree.  So there is an $\mathcal{O}(N)$ method if it
is known that in the resulting DAG each non-source node has one
parent.

It is possible to expand this method and test to see if the DAG is a
tree.  If the great majority of Jaywalks to be processed are tree, it
might be worth speculatively reconstructing for a tree, and throwing
away the work if the DAG is not one.  The expansion of the method is
straightforward.  Recall that the BNB pair is the rightmost parent
viewed from the perspective of the node in the dominance drawing.  If
we exchange principal and converse ranks and find the BNB pair for
each node in the modified Jaywalk, we find the leftmost parents. If
and only if all leftmost parents are the same as the rightmost
parents, the DAG is a tree.

## Small Jaywalks

The tree algorithm can potentially be used as the basis for a
relatively efficient methods for small trees when the converse sort is
known.  It would work for mostly-tree Jaywalks, but have worst case
$\mathcal{O}(N^2)$ performance.  The idea for this is straightforward.
Consider the exhaustive method.  Instead of this, scan for parents of
every node.  The candidate parents are all nodes with lower principal
rank.  If we first use the aforementioned method to find the left and
rightmost parents, we can further reduce the range of principal rank
of potential parents.  In other words we scan exhaustively between the
nodes that are these extreme parents.  We now have a real algorithm
for small Jaywalks in four phases.

1. For each node fins the rightmost parent using the BNB method for
   finding BNB pairs.
2. For each node find the leftmost parent.  Use the method for finding
   BNB pairs, but with principal and converse ranks exchanged.
3. Scan for parents in reverse order from the leftmost parent to the
   rightmost parent.  For all nodes in between that are lower in
   converse rank than the child, add a parent if it increases the
   converse rank.  The rightmost parent has the highest converse rank
   and lowest principal rank.  (This step can be potentially merged
   with step 2.)
4. Find each node's children by removing child-to-parent links.  This
   can potentially be merged with step 3.

This algorithm cannot be used directly for subgraph reconstruction.
This is because we would need to know the subsort of the subgraph by
converse value.  Thus it cannot be used as a first phase in a
divide-and-conquer reconstruction. (There is a possibility that
subsorts could be created from the full sort. This is a topic for
future exploration.)

## Middle children

Before moving on to a comprehensive algorithm, we finish coverage of
construction based on trees.  In the same say that ANA pairs are like
BNB pairs, we can exchange children with parents, rotating the
dominance drawing 180 degrees.  Thus with two more passes we can find,
for every node, the leftmost and rightmost child.  This would
construct the DAG for many if not most Jaywalks that we expect to
encounter.  However, it would not find edges that are both middle
children and middle parents.  Furthermore, we have not yet found an
efficient way to detect when the method falls short.

# Core algorithm

## The eyes have it

Let us set aside any converse sort, and assume that we only have rank
pairs with which to work.  We allow ourselves the convenience of
having the pairs listed in order of principal rank.  Recall that ranks
do not have to be continuous integer ranges.  We argued earlier that,
since such a construction algorithm will implicitly sort the nodes by
converse rank, the minimum complexity is $\mathcal{O}(N\log(N))$.
Furthermore, since mergesort is the most common sort algorithm, it
makes sense to explore similar divide-and-conquer methods.  This leads
us to examine parts of subgraphs.  First we divide up the problem and
more-or-less completely "solve" (construct) for each
sub-problem. Second, a brief investigation confirms that we will need
some kind of merging of sequences of parts.  We will expand on this
shortly.  Third, a brief investigation indicates that the banba chains
might be useful.

<!-- Export at 80% -->

![An illustration of what we call a DAG *segment*, a selected portion
of Jaywalk DAG that has no nodes to the left or right, but may have
nodes to above and below, including above-left and so on.  Let P and Q
be two nodes within the segment with the largest and smallest converse
tanks.  From these we create a continuous border using the ANB and BNB
chains from P and the ANA and BNA chains from Q. The chains intersect
at the lowest and highest principal ranks (X,Y).  We call this border
the *eye* of the segment.  We call the parts of the chains that are
within the segments the *main* chains, and we call the remainder of
the chains beyond the intersections their *continuations*.  Because
there are no nodes to the left and right of the segment, the DAG
itself provides two of the pairings.  The ANA pairs are the rightmost
children of each node's connections, and the BNB pairs are the
rightmost parents of a node's
connections.\label{figB}](figs-dagify/Dag-B.svg)

We choose a particular approach, and we we begin by focusing on
segments of DAGs and an envelope around them that we will call there
*eyes*.  Suppose that we have completely constructed the sub-DAG for a
(sort-wise) continuous range of principal ranks.  This is our
divide-and-conquer sub-problem.  In addition, we know the BNB, ANA, ANB
and BNA pairs for all nodes in the sub-problem.  Next we further
subdivide the DAG into segments, each with (sort-wise) continuous
ranges of converse ranks.  Figure \ref{figB} shows the nodes (but not
edges) of a DAG segment.  Our treatment of segments uses the nodes
with highest (P) and lowest (Q) converse ranks.  We trace along the
ANB pairs from P and ANA pairs from Q to make chains that cross at the
node with the highest principal rank in this segment (Y).  We call the
chains within the segments the *main* chains.  The chains may have
continuations if the sub-DAG has nodes to the right above and/or
below.  In like fashion we trace out chains using BNB pairs from P and
BNA pairs from Q.  These meet at the node with the lowest principal
rank (X), possibly continuing on to nodes outside of this segment.  We
say that these four main chains make the *eye* of the segment.


## Division into segments

<!-- Export at 80% -->

![An example merge that creates the DAG connections between left and
right blocks.  The process is much like that in a merge sort,
intermeshing segments on the left and right.  Segments with more than
one node are shown with bounding boxes and eyes as in figure
\ref{figB}.  A few nodes are highlighted by rendering with solid
circles.  Node P is a parent of Q, highlighting the fact that segment
eyes do not isolate within a block.  In contrast, it is never possible
for a node within an eye on one side to have an edge connection to the
other block.  That is, all edges that cross the boundary between the
blocks are between nodes on eyes.  Nodes in an ANB chain on the left
are parents to all the nodes in the main BNA chain of the next higher
segment on the right.  For example, A is a parent to C.  But more than
that, A is a parent to D on the continuation of that left BNA chain.
Therefore continuation chains, at least on one side, have to be
considered when constructing all the edges that join nodes between the
blocks.\label{figA}](figs-dagify/Dag-A.svg)

Segments, along with the chains that make up their eyes, are the main
building blocks for our algorithm.  Now consider how we combine two
sub-problems into one.  This is illustrated in figure \ref{figA}.  Suppose
that we have separately completed two sub-DAGs that have adjacent
ranges of principal rank.  We handle the process of combining these
much like a mergesort of the converse ranks.  That is to say we
intersperse segments in the left block with segments in the right
block.  The ranges of converse ranks in each segment are maximized
while not overlapping.

## Edge additions

A complete sub-DAG has all edges in lists of children and parents.  It
also ensures that we have banba pairs for every node.  Consider the
updates needed to edges.  First note that there is no need to remove
edges since the combined DAG has all the edges of the two separate
blocks (DAGs).  Second note that all additional edges "cross the line"
between the blocks.  A vertical line is drawn in figure \ref{figA} to
illustrate this boundary.  Some care is needed when identifying new
edges.  Within the blocks the segment eyes do not limit connections.
Node P in the illustration is a parent of Q even though both are
inside eyes.  Between the left and right blocks, however, the eyes are
more helpful.  All additional edges are between a node on the main ANB
chain for a segment on the left and a node on the main BNA chain for a
node on the right.  Therefore we can find all additional edges by
taking each segment in the left block in turn and considering all
segments above it in the right block.  We can refine this further.
Suppose that we are adding edges for the segment in the illustration
containing A on the left.  All nodes on its main ANB chain (A, E and
F) are parents to all nodes in the main BNA chain of the next block on
the right (B and C).  We also have to consider segments above on the
right.  However, we can instead restrict this to the chain that starts
at B as it continues past C.  In this case D is a child of A and F but
not of E, because D can be reached from E via G.

![One approach to finding all additional edges in a Jaywalk DAG when
combining a left block and a right block when each is a self-contained
DAG.  As was illustrated in figure \ref{figB}, all new edges must be
from the ANB of an eye on the left to a node in a BNA chain in an eye
on the right.  Therefore we can consider the segments one at a time on
the left.  In the manner of a merge sort we can focus on the next
higher segment on the right, and start making connections there.
Consider node P.  Its current ANA pair is its current rightmost child.
To this children C, B and A are added.  This automatically updates its
ANA pairing to A.  The first new child for both Q and R is E.
Likewise, the new children for S are E, D, C, B and A.  Observe that
the old ANA pair for S serves as the upper limit on the converse rank
for its new children.  The minimum set of children is the main BNA
chain in the next right segment.  Also note that the nodes that limit
the range of child nodes themselves are members of an ANA chain, and
are guaranteed to increase in converse rank.  After processing this
left segment, P is the rightmost parent of A, B and C and therefore
the new BNB pairing for all three.  Furthermore, we can consider the
right segment "done" insofar as no further edges will be added to it.
(This assumes that we process segments from bottom to
top.)\label{figC}](figs-dagify/Dag-C.svg)

This is broken down in detail in figure \ref{figC}.  The example
recycles the node lettering.  Nodes P through S are parents of A
through C, 12 basic edges.  These are the main ANB and BNA segment
chains. As just proven, we will find all DAG edges if we restrict
ourselves to P through S and consider the BNA chain continuation of
the right segment.  For Q through S we add edges starting at A and
stopping at E.  The topping point can be found by looking at the ANA
pairings on the left.  For instance, Q and R are paired with the same
node, and its converse rank is greater than that of node E but less
than that of F.  The ANA pair for P has a lower converse rank, and it
only pairs with the nodes on the main chain.

Various minor strategies can be used in the implementation.  For
example, one can append the children for P.  Then for Q one can first
append the extra nodes and second copy the tail of P's children.

## Updating pairings

In figure \ref{figC} observe that the ANA pairing is also the last
child.  Therefore it is updated for each node as a by-product of
adding children.  There is no need to store these separately.  Also
note that there is no significant danger of confusion between a node's
ANA pair before and after merging, since we only use it during the
specific process of adding children.  These implicit updates are
performed for each segment on the left in turn.  The updated pairings
are always to a node (the one with the lowest converse rank) in the
immediately next higher segment on the right.

![Update to the ANB and BNA pairings when blocks are merged.  In order
to add the extra DAG edges, which is the main aim of such a merge, as
illustrated in figure \ref{figC}, we want the ANB and BNA chains,
because these provide each node's sequences of children and parents.
If we need ANA or BNB pairs, these are available from the DAG.  In
other words, a side effect of adding edges is that ANB and BNA pairs
are used to update ANA and BNB pairs.  In contrast, as illustrated
here, we use ANA and BNB chains to update the ANB and BNA pairs.  This
is performed, as we traverse segments, for the join between a segment
on the right and the next immediately higher segment on the left.
Example connections are shown.  The node A currently has ANB pair x,
and this needs to be updated to T.  The node Q currently has BNA pair
Y, and this needs to be updated to A.  Hence the complete updates have
two parts.  We traverse the main ANA chain of the left segment,
assigning the ANB pairs for A, B and C to T.  Also we traverse the
main BNB chain of the right segment, assigning the BNA pairs for P, Q,
R, S and T to A.\label{figD}](figs-dagify/Dag-D.svg)

In addition we need the ANB and BNA pairs for each node.  The overall
algorithm considers segments in turn in increasing converse rank.  As
described above, we do the work for edge additions for a segment on
the left and the next on the right.  Then we work with the same
segment on the right and the next segment on the left.  The work in this
case is updating the ANB and BNA pairs.  Thus we alternate between
left and right segments in turn.

Figure \ref{figD} illustrates the updates required.  Simply put, we
redirect the ANB pairs for all nodes on the main ANA chain on the left
to the same node on the right.  This is the node in the right segment
with the highest converse rank.  The BNA updates are in essence the
same, with 180-degree rotation.  All nodes in the main BNB chain on
the right are updated to the same BNA pair on the left.

Finally we need to check on the BNB pairs.  We can use the last parent
and not store separately.  This is a side benefit of our
segment-by-segment approach.  Look back at the earlier figures.  The
only node on a segment BNB chain that has its BNB pair updated is the
last one on the main chain. Moreover, all updates are to a node with
converse index below the segment.  So the test for the end of a
segment's main BNB chain remains unchanged.

## Rewind


The basic framework for our algorithm is that we maintain ANB and BNA
pairs, and to update these we need ANA and BNB chains.  To update the
ANA and BNB chains we use the ANB and BNA chains.  The building of DAG
edges also uses the ANB and BNA chains.  We have carefully shown that
the ANA and BNB pairs can be obtained from the DAG edge children and
parents lists.

The natural question is the how much work is involved?  Three processes
are involved, namely the update process, the edge addition process and
the segment identification process.  The actual segment traversal is
counted in the first two processes.  The edge addition process is
$\mathcal{O}(E)$, and this is essential.  Our algorithm proceeds
exactly as a mergesort, and the interlacing is exactly the same
process as a merge.  It costs us nothing extra to maintain linked
lists of nodes in order of converse rank.  Identifying the segments in
a standard mergesort in essence involves traversing each segment node
to find the end of the segment.  Look back at figure \ref{figB}.  We
can accelerate the traversal from Q to X or Y.  However we can only
find P by continuing node by node through the upper part of the eye,
because the BNB and ANB chains are in the "wrong" direction.  This is
unavoidable, because we cannot break the bound on worst-case
mergesort.  Indeed, this bound on the average performance applies to a
set of Jaywalks if every Jaywalk is equally likely.
However, if we know the sort, we can reduce the work of identifying segments
to the number of segments.  (This is discussed later.)

The remaining complexity question concerns that of the ANB and BNA
updates.  We can bound this.  Each pairing can be updated once per
merge layer.  Therefore the work is bounded by
$\mathcal{O}(N\log(N))$.  This will occur for the worst-case number of
segments, which is when every segment has one node.  The simplest
version of this is bit reversal.  This is when, for say size 16, the
converse rank is the bit reversal of the principal rank.  This
generates a DAG with a large number of edges.  Other Jaywalks with all
single-node segments are not quite as extreme.  Either way, it is not
immediately clear if the ANB / BNA updates can involve much more work
that the number of edges.  We leave a comprehensive exploration of the
complexity of the updates to a future study.  In the meantime we
expect at least the average complexity for trees to be much better
than $\mathcal{O}(N\log(N))$ if and only if prior knowledge of the
sort order is available and is exploited.
