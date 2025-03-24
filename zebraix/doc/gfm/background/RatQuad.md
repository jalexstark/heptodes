---
author:
- J. Alex Stark
date: 2003–2022
title: Rational Quadratic Splines and Bézier Splines
---

- [<span class="toc-section-number">1</span> Task](#task)
  - [<span class="toc-section-number">1.1</span> General
    purpose](#general-purpose)
  - [<span class="toc-section-number">1.2</span> Specific
    aims](#specific-aims)
- [<span class="toc-section-number">2</span> Topic: Matching
  splines](#topic-matching-splines)
- [<span class="toc-section-number">3</span> Splines
  foundation](#splines-foundation)
  - [<span class="toc-section-number">3.1</span> Matching spline
    specification](#matching-spline-specification)
- [<span class="toc-section-number">4</span> Topic: Path-invariant
  bilinear
  transformation](#topic-path-invariant-bilinear-transformation)
- [<span class="toc-section-number">5</span> Topic: Arc
  angles](#topic-arc-angles)
  - [<span class="toc-section-number">5.1</span> Quarter circles and
    semicircles](#quarter-circles-and-semicircles)
  - [<span class="toc-section-number">5.2</span> General circular
    arc](#general-circular-arc)
  - [<span class="toc-section-number">5.3</span> Most-circular
    elliptical arc](#most-circular-elliptical-arc)
- [<span class="toc-section-number">6</span> Topic: Practical method for
  finding polynomials](#topic-practical-method-for-finding-polynomials)
  - [<span class="toc-section-number">6.1</span> Numerical
    generality](#numerical-generality)
- [<span class="toc-section-number">7</span> Topic: Converting and
  matching](#topic-converting-and-matching)
  - [<span class="toc-section-number">7.1</span> Use cases](#use-cases)
  - [<span class="toc-section-number">7.2</span> Denominator
    balancing](#denominator-balancing)
    - [<span class="toc-section-number">7.2.1</span> Scope of ellipse
      conversion](#scope-of-ellipse-conversion)
    - [<span class="toc-section-number">7.2.2</span> Ellipse conversion
      method](#ellipse-conversion-method)
  - [<span class="toc-section-number">7.3</span> Constraints and
    conversion to parabolas](#constraints-and-conversion-to-parabolas)
  - [<span class="toc-section-number">7.4</span> Intersection-angle
    form](#intersection-angle-form)
  - [<span class="toc-section-number">7.5</span> Summary](#summary)
- [<span class="toc-section-number">8</span> Topic: Choice of
  specification schemes](#topic-choice-of-specification-schemes)
  - [<span class="toc-section-number">8.1</span>
    Requirements](#requirements)
  - [<span class="toc-section-number">8.2</span> Specification
    schemes](#specification-schemes)
  - [<span class="toc-section-number">8.3</span> Convenience features of
    intersection-angle
    scheme](#convenience-features-of-intersection-angle-scheme)
- [<span class="toc-section-number">9</span> Topic: Point distribution,
  parameterization and
  attachment](#topic-point-distribution-parameterization-and-attachment)
- [<span class="toc-section-number">10</span> Other design choices and
  considerations](#other-design-choices-and-considerations)
  - [<span class="toc-section-number">10.1</span> Stretch
    factors](#stretch-factors)

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

# Task

## General purpose

Zebraix is intended to be a focused utility. In its unextended version
it is restricted to the specifics needed for a limited purpose.
Nonetheless, we do not want it to be unnecessarily narrow. For example,
we would like to be able to draw vertex shapes other than circles. The
goal of this document is to explore design possibilities that meet some
fairly basic desires, and to do so in a way that is flexible. Or put
another way, can we leverage the capabilities of libraries that we are
using anyway, without adding a lot of complexity to Zebraix, and gain
useful features?

We can describe two motivating desires, in somewhat technical fashion,
as follows.

1.  We would like to be able to construct shapes from the affines
    transformation of circle arcs.

2.  We need to be able to extract points along curve paths, specifying
    via a parameter, such that the locations are evenly distributed.

Why these technical needs? They both have a mix of functional benefits.
Consider the second one. Even if we restricted Zebraix to one or a few
arrow types and to circles with external labels, we would still need to
attach the edges to the circles. We would benefit from being able to lay
out edge attachment points automatically. This benefit would grow
quickly with an only modest expansion of vertex shapes to rectangles, to
rounded rectangles parallelograms, and so on. Cubic splines provide an
underlying mechanism for this, but circles and more general arcs are
challenging. Zebraix could take a hard-coded approach to circles and
rectangles. Here we explore what it would take to be more flexible.

For the foreseeable future Zebraix’s rendering will be primarily to SVG.
Since any SVG renderer, like Cairo, has to handle text, we assume that
it will handle cubic splines and elliptical arcs. We want to avoid
adding a lot of complexity to Zebraix. We might characterize our present
discussion asking what it would take to “teach Zebraix” about affine
transformations and circular arcs, and hence also affine transformations
of circular arcs. This assume that Zebraix already “knows” about cubic
splines.

This document is not short. Nonetheless, the solutions that we propose
are not especially complex or extensive to code, and they are
approachable and practical for the user.

## Specific aims

Cubic splines can, as mentioned, above, be readily split parametrically
into pieces and otherwise be joined, transformed, and specified quite
elegantly. On the other hand, elliptical arcs, and indeed segments of
conic sections more generally, are a bit more tricky, even when
expressed as rational polynomials. Consequently this document is really
about rational quadratic splines and how they might be used in the
context of Zebraix. We explore the topics that follow below. These
correspond approximately to sections. From henceforth we will often
abbreviate *rational quadratic spline* to *RQS*.

- Matching RQSs to cubic splines. Particular attention paid to
  parabolas, we are common to both, and to matching velocities.

- Path redundancy in RQSs, and bilinear transformation.

- Conversion of RQSs to elliptical arcs, when appropriate for rendering
  engines.

- Schemes for distributing points along RQSs.

- Specification of RQSs in ways that are invariant under affine
  transformation.

- Conversion of RQSs to approximating cubic splines when the equivalent
  elliptical arc has a very large axis.

# Topic: Matching splines

Familiarity with the content of this section iss assumed in subsequent
sections. We assume familiarity with the basics of cubic Bézier splines,
but will try to give a more standalone discussion of rational quadratic
splines. Understanding of conic sections is assumed.

# Splines foundation

We will base all our discussion on two defining equations: one for cubic
splines, and one for RQSs. The cubic spline is defined with respect to
$t$, which has the range $[0, 1]$ over the path. The path and its
derivative are

$$
g(t) = (1-t)^3p_0 + 3t(1-t)^2p_1 + 3t^2(1-t)p_2 + t^3p_3 \text{.} \label{Eq01}
$$

$$
g'(t) = 3(1-t)^2(p_1-p_0) + 6t(1-t)(p_2-p_1) + 3t^2(p_3-p_2) \label{Eq02}
$$

For RQSs, we use a three-point expression along with three weights
($w_A$, $w_B$, $w_C$). The path and its derivative are

<!-- mdformat off (Document metadata) -->

$$
f(t) = \frac{%
w_A(1-t)^2p_0 + 2w_Bt(1-t)p_m + w_Ct^2p_3
}{%
w_A(1-t)^2 + 2w_Bt(1-t) + w_Ct^2
} \label{Eq03}
$$

$$
f'(t) = \frac{%
2w_Aw_B(1-t)^2(p_m-p_0) + 2w_Aw_Ct(1-t)(p_3-p_0) + 2w_Bw_Ct^2(p_3-p_m)
}{%
(w_A(1-t)^2 + 2w_Bt(1-t) + w_Ct^2)^2
}\label{Eq04}
$$

<!-- mdformat on -->

There is redundancy in the weights: they can be multiplied by an
arbitrary scaling factor.

<figure>
<img src="figs-ratquad/RatQuad-A.svg"
alt="Examples of cubic splines and rational quadratic splines (RQSs) that fit into the same parallelogram. The top-left curve is cubic with control points evenly spaced with respect to the end points. In the next the control points are unevenly spaced. The top-right curve illustrates the ability of cubic cplines to take an S shape. (Pandoc has no paragraph break in captions.) The bottom-left curve is a parabole, and thus is both a cubic spline and a RQS. The next is an elliptical RQS, and one that is like a quadrant in that it is the affine transformation of a circle quadrant. Finally, in the bottom-right the curve is also an elliptical arc, this one with a flatter shape." />
<figcaption aria-hidden="true">Examples of cubic splines and rational
quadratic splines (RQSs) that fit into the same parallelogram. The
top-left curve is cubic with control points evenly spaced with respect
to the end points. In the next the control points are unevenly spaced.
The top-right curve illustrates the ability of cubic cplines to take an
S shape. (Pandoc has no paragraph break in captions.) The bottom-left
curve is a parabole, and thus is both a cubic spline and a RQS. The next
is an elliptical RQS, and one that is like a quadrant in that it is the
affine transformation of a circle quadrant. Finally, in the bottom-right
the curve is also an elliptical arc, this one with a flatter
shape.</figcaption>
</figure>

The gradients at the end points are useful for interpreting the
specifications.

$$
g'(0) = 3(p_1-p_0) \label{Eq05}
$$

$$
g'(1) = 3(p_3-p_2) \label{Eq06}
$$

$$
f'(0) = \frac{2w_B}{w_A}(p_m-p_0) \label{Eq07}
$$

$$
f'(1) = \frac{2w_B}{w_C}(p_3-p_m) \label{Eq08}
$$

So both forms of spline have readily explicable path directions at the
end points. Cubic splines have a velocity interpretation that is
somewhat understandable, but the three weights in the RQS expression are
not very helpful.

## Matching spline specification

We now proceed to formulate a RQS specification that matches that of
cubic splines. We recommend this for consideration as one method for a
real software package.

Our general approach is to narrow cubic splines to those for which $p_m$
exists. This is the intersection of the lines from $p_0$ to $p_1$ and
$p_3$ to $p_2$. We then formulate a four-point specification for RQSs
that match the cubic velocities at either end, and that can be identical
for parabolas. In a sense $p_1$ and $p_2$ subdivide the lines to $p_m$.
Let us, with this in mind, define proportions $\lambda$ and $\mu$
according to the following.

$$
p_1-p_0 = \lambda(p_m-p_0) \label{Eq09}
$$

$$
p_2-p_3 = \mu(p_m-p_3) \label{Eq10}
$$

We can match gradients, that is

$$
g'(0) = f'(0) \label{Eq11}
$$

$$
g'(1) = f'(1) \label{Eq12}
$$

if

$$
\lambda = \frac{2w_B}{3w_A} \label{Eq13}
$$

$$
\mu = \frac{2w_B}{3w_C} \label{Eq14}
$$

Aside: we can the rewrite the cubic spline gradients as

$$
g'(t) = 3\lambda(1-t)^2(p_m-p_0) + 6t(1-t)((1-\mu)p_3-(1-\lambda)p_0+(\mu-\lambda)p_m) - 3\mu t^2(p_m-p_3) \label{Eq15}
$$

There should be an exact match for parabolas, since these can be
expressed as cubic splines and RQSs. The following hold for parabolas,
simplifying both $f(t)$ and $g(t)$, and tying in with our subdivision
proportions $\lambda$ and $\mu$.

$$
\lambda = \mu = \frac23 \label{Eq16}
$$

$$
w_A = w_B = w_C \label{Eq17}
$$

$$
f(t) = g(t) = (1-t)^2p_0 + 2t(1-t)p_m + t^2p_3 \label{Eq18}
$$

This general approach works well when it works. It works at infinity. It
does not work will for near-parallel lines. Put more concretely, it
works when the lines from the end points to $p_m$ are parallel and
perpendicular to the line from $p_0$ to $p_3$. Then $p_m$ is at
infinity. But the beauty of this approach is that that is OK. We can
specify a semicircle (or half-ellipse) very conveniently in this way. On
the other hand, when $p_0$, $p_3$ and $p_m$ are colinear, or nearly so,
then the ambiguity in specifying $p_m$ via $p_1$ and $p_2$ is a problem.

# Topic: Path-invariant bilinear transformation

The RQS representation incorporates redundancy. The formula in terms of
three weights is specified by 3 points and 3 weights. We can remove 1
weight as a degree of freedom, since scaling the numerator and
denominator by the same value leaves $f(t)$ unchanged. Suppose that
$f(t)$ produces and elliptical arc. We can specify an ellipse by its
centre, its 2 axis lengths, and a rotation. These degrees of freedom are
equivalent to 2 and half points. We are left with 3 degrees. The
beginning and ending angles of the arc account for 2 of these. But one
degree of freedom is left over. What does this control?

The answer is in a bilinear transformation of $t$ that leaves the path
unchanged but changes the speed pattern with which we traverse it. Let

Equation :Eq36.

$$
v(t) = \frac{\sigma t}{1+(\sigma -1)t} \label{Eq36}
$$

For a control parameter $\sigma$. This is basically the general form of
a bilinear transformation for which $v(0)=0$ and $v(1)=1$. The path
formula is modified as follows.

$$
f_\sigma(t) = f(v(t)) =\frac{%
w_A(1-t)^2p_0 + 2\sigma w_Bt(1-t)p_M + \sigma^2 w_Ct^2p_3
}{%
w_A(1-t)^2 + 2\sigma w_Bt(1-t) + \sigma^2 w_Ct^2
} \label{Eq37}
$$

Consider the velocity of the modified path at either end.

$$
f'_\sigma(0) = \left(\frac{2\sigma w_B}{w_A}\right)(p_m-p_0) \label{Eq38}
$$

$$
f'_\sigma(1) = \left(\frac{2w_B}{\sigma w_C}\right)(p_3-p_m) \label{Eq39}
$$

$$
f'_\sigma(0)f'_\sigma(1) = f'(0)f'(1) \label{Eq40}
$$

We can make various observations. First, we can use $\sigma{}^{-1}$ in
place of $\sigma$ to create an inverse modification. More generally, if
we transform by $\sigma_1$ and then by $\sigma_2$, the overall
transformation is the same as for $\sigma=\sigma_1\sigma_2$. Second, for
$\sigma>1$, the transformation speeds the path traversal wrt $t$ at the
beginning and slows it at the end. The product of the two end velocities
remains unchanged. This is useful to Zebraix, because it provides a
means for redistributing anchor points along a curve.

Changing the path in this way involves merely moving $p_1$ and $p_2$
such that the lengths of $p_0$ to $p_1$ , and of $p_3$ to $p_2$, are
multiplied by reciprocal amounts. In other words, we scale $\lambda$ and
$\mu$ in opposite ways. In some cases, especially when manipulating
RQSs, it is useful to have $w_A=w_C$, and equivalently $\lambda=\mu$.
This simplifies the denominator of the RQS and creates a canonical form
for a path. We call this *rebalancing*.

<figure>
<img src="figs-ratquad/RatQuad-B.svg"
alt="Example elliptical arcs and the distribution of points, shown with t evenly spread, along the paths. The top-left is a semicircle and the vertical locations of points are the sines of points evenly distributed by tangent. The middle-top picture is of half and ellipse, and really only the positions of p_1 and p_2 are changed. We can further change them (top-right) such that the shape is unchanged but the points are distributed asymmetrically wrt t. (Pandoc has no paragraph break in captions.) The two lower curves are quarter-ellipses. The left can be considered a stretched square, with the control points stretched in like manner. The right has equal path velocities at either end, as might be chosen if a fairly even spacing of points were desired." />
<figcaption aria-hidden="true">Example elliptical arcs and the
distribution of points, shown with <span
class="math inline"><em>t</em></span> evenly spread, along the paths.
The top-left is a semicircle and the vertical locations of points are
the sines of points evenly distributed by tangent. The middle-top
picture is of half and ellipse, and really only the positions of <span
class="math inline"><em>p</em><sub>1</sub></span> and <span
class="math inline"><em>p</em><sub>2</sub></span> are changed. We can
further change them (top-right) such that the shape is unchanged but the
points are distributed asymmetrically wrt <span
class="math inline"><em>t</em></span>. (Pandoc has no paragraph break in
captions.) The two lower curves are quarter-ellipses. The left can be
considered a stretched square, with the control points stretched in like
manner. The right has equal path velocities at either end, as might be
chosen if a fairly even spacing of points were desired.</figcaption>
</figure>

Some bilinear transformations are illustrated In each case points along
the paths are shown, for even distribution of $t$. If we simply scale
the figure, we can change a circle into an ellipse, leaving the pattern
of points essentially unchanged. We can also modify the relative
distances of the control points from the end points. The illustrated
quarter-ellipse curves show how the bilinear transformation provides a
means for spreading the anchor points according to need. For example,
one might want a distribution that is even in one axis. Alternatively,
as shown, one might prefer a distribution that is even along the path
direction.

Linear RQSs are basically squashed elliptical arcs, with parabolas as an
extreme case. By *squashed* we mean that the points on the curve are
transformed such that they lie on the line between $p_0$ and $p_3$. Put
another way, it is like decomposing locations into components parallel
and perpendicular to that line, and zeroing the perpendicular component.
The drawings along the top of the figure illustrate this, with the
distribution of vertical locations shown. We can choose a distribution
of points that is symmetric or with a chosen degree of asymmetry. We can
also choose the central spread.

A key feature of this four-point specification of RQSs is that it is
consistent under affine transformation. While an affine transformation
may result in unequal changes to the distances to control points, the
RQS is consistently transformed.

# Topic: Arc angles

## Quarter circles and semicircles

<figure>
<img src="figs-ratquad/RatQuad-D.svg"
alt="Quarter-circle and semicircle surves. The left curve shows the control points for a quarter circle, for which the RQS formula combines the tan-half-angle formulae for sine and cosine. If we extend the range of tan half angles to [-1,1], we get a semicircle. RQS paths require a parameter in the range [0,1], but we can obtain the wider range by simple scale and offset. The corresponding control points are shown for the right curve." />
<figcaption aria-hidden="true">Quarter-circle and semicircle surves. The
left curve shows the control points for a quarter circle, for which the
RQS formula combines the tan-half-angle formulae for sine and cosine. If
we extend the range of tan half angles to <span
class="math inline">[−1,1]</span>, we get a semicircle. RQS paths
require a parameter in the range <span class="math inline">[0,1]</span>,
but we can obtain the wider range by simple scale and offset. The
corresponding control points are shown for the right curve.</figcaption>
</figure>

A RQS can be used to generate a quarter circle if it is formulated in
terms of tan half angles. That is, we can write

$$
f_Q(t) = \begin{bmatrix}
\frac{1-t^2}{1+t^2} \\
\frac{2t}{1+t^2}
\end{bmatrix} \label{Eq34}
$$

since $t$ is in the range $[0,1]$. This is illustrated\text{ (see figure
). We can write this in termsof $p_m$ and so on, and we can characterize
in terms of control points.

$$
f_Q(t) = \frac{%
(1-t)^2\begin{bmatrix}1\\0\end{bmatrix} + 2t(1-t)\begin{bmatrix}1\\1\end{bmatrix} + 2t^2\begin{bmatrix}0\\1\end{bmatrix}
}{%
(1-t)^2 + 2t(1-t) + 2t^2
} \label{Eq41}
$$

We can also extend the curve to a semicircle. If we do this by extending
the tan-half-angle range to $[-1,1]$, we can keep $t$ to its RQS range
by using $2t-1$.

<!-- mdformat off (Document metadata) -->

$$
f_S(t) = \begin{bmatrix}
\frac{1-(2t-1)^2}{1+(2t-1)^2} \\
\frac{2(2t-1)}{1+(2t-1)^2}
\end{bmatrix}  \label{Eq42}
$$

$$
f_S(t) = \begin{bmatrix}
\frac{2t(1-t)}{t^2+(1-t)^2} \\
\frac{t^2-(1-t)^2}{t^2+(1-t)^2}
\end{bmatrix} \label{Eq35}
$$

<!-- mdformat on -->

While we can write a quarter-circle RQS in terms of $p_m$, this is not
possible for a semiscircle RQS, since there is no intersection.
Nonetheless, we can get close.

$$
f_S(t) = \frac{%
(1-t)^2\begin{bmatrix}0\\-1\end{bmatrix} + 2t(1-t)\begin{bmatrix}1\\0\end{bmatrix} + t^2\begin{bmatrix}0\\1\end{bmatrix}
}{%
(1-t)^2 + (0)2t(1-t) + t^2
}  \label{Eq43}
$$

## General circular arc

<figure>
<img src="figs-ratquad/RatQuad-E.svg"
alt="A general view of a circular arc, paramaterized by the radius and tan of the half angle. The triangle that contains the RQS is drawn separately to show the side lengths and relative location of p_1 and p_2." />
<figcaption aria-hidden="true">A general view of a circular arc,
paramaterized by the radius and tan of the half angle. The triangle that
contains the RQS is drawn separately to show the side lengths and
relative location of <span
class="math inline"><em>p</em><sub>1</sub></span> and <span
class="math inline"><em>p</em><sub>2</sub></span>.</figcaption>
</figure>

In like manner to the semicircle we can create a symmetric circular arc
RQS as illustrated. Let$\delta$ be the tangent of half the arc angle. If
we take the quarter circle RQS and replace $t$ with $\delta t$.

$$
f_\delta(t) = \frac{%
(1-t)^2\begin{bmatrix}r\\0\end{bmatrix} + 2t(1-t)\begin{bmatrix}r\\r\delta\end{bmatrix} + (1+\delta^2)t^2\left(\frac{1}{1+\delta^2}\right)\begin{bmatrix}r(1-\delta^2)\\2r\delta\end{bmatrix}
}{%
(1-t)^2 + 2t(1-t) + (1+\delta^2)t^2
} \label{Eq44}
$$

While this has some fussy details, it is what we expect in terms of
$p_3$, and so on. Now let us consider the velocities at either end.
These are those at the original locations in the quarter-circle RQS, but
scaled by $\delta$ since we replaced by $\delta t$.

$$
f'_\delta(t) = r\delta f'_Q(\delta t)\label{Eq45}
$$

$$
f'_Q(t) = 2\begin{bmatrix}
\frac{-2t}{(1+t^2)^2} \\
\frac{1-t^2}{(1+t^2)^2}
\end{bmatrix} \label{Eq46}
$$

$$
f'_\delta(0) = 2r\delta\begin{bmatrix}
0\\1
\end{bmatrix} \label{Eq47}
$$

$$
f'_\delta(1) = \left(\frac{2r\delta}{1+\delta^2}\right)%
\begin{bmatrix}
\frac{-2\delta}{1+\delta^2} \\
\frac{1-\delta^2}{1+\delta^2}
\end{bmatrix} \label{Eq48}
$$

$$
\left| f'_\delta(1) \right| = \frac{2r\delta}{1+\delta^2} \label{Eq49}
$$

If $p_1$ and $p_2$ are our RQS control points located by $\lambda$ and
$\mu$, then the product of the velocities can be matched.

$$
3\lambda\left|p_m-p_0\right|\,3\mu\left|p_m-p_3\right| =
\left|f'_\delta(0)\right|\,\left|f'_\delta(1)\right| \label{Eq50}
$$

$$
9\lambda\mu(r\delta)^2 = \frac{4(r\delta)^2}{1+\delta^2} \label{Eq51}
$$

Thus

$$
\lambda\mu = \frac{4}{9(1+\delta^2)} \label{Eq52}
$$

Since $\cos(\Omega) = -\cos(2\varphi) = -(1-\delta^2)/(1+\delta^2)$,

$$
\lambda\mu = \frac{2}{9}(1-\cos(\Omega)) \label{Eq53}
$$

However, $\lambda\mu = 1$ for the semi-circle case, so we would like a
different characterization.

## Most-circular elliptical arc

<figure>
<img src="figs-ratquad/RatQuad-F.svg"
alt="Illustration of circular arc RQS (top-left) and an RQS found by affine transformation (top-middle). The top-right drawing shows an alternative pair of locations for the control points based on the a-b-c formulae in the text. The bottom row shows a similar set of RQSs, but for a quarter ellipse." />
<figcaption aria-hidden="true">Illustration of circular arc RQS
(top-left) and an RQS found by affine transformation (top-middle). The
top-right drawing shows an alternative pair of locations for the control
points based on the a-b-c formulae in the text. The bottom row shows a
similar set of RQSs, but for a quarter ellipse.</figcaption>
</figure>

Let

$$
a = \left|p_3-p_0\right| \label{Eq64}
$$

$$
b = \left|p_m-p_0\right| \label{Eq65}
$$

$$
c = \left|p_m-p_3\right| \label{Eq66}
$$

Suppose, as illustrated, we take a circular arc and apply an affine
transformation that preserves the angle $\Omega$. Suppose further that
we place $p_1$ and $p_2$ such that

$$
\left|p_1-p_0\right| = \frac13(a+b-c) \label{Eq54}
$$

$$
\left|p_2-p_3\right| = \frac13(a-b+c) \label{Eq55}
$$

Then the product of the velocities is

$$
9\left|p_1-p_0\right|\,\left|p_2-p_3\right| =
2bc-(b^2+c^2-a^2) \label{Eq56}
$$

$$
9\left|p_1-p_0\right|\,\left|p_2-p_3\right| =
2bc(1-\cos(\Omega)) \label{Eq57}
$$

$$
9\left|p_1-p_0\right|\,\left|p_2-p_3\right| =
2\lambda\mu bc \label{Eq58}
$$

This looks as if we just got back where we started, but not quite.
Suppose that we start with a circular arc RQS based on $p_1$ and $p_2$
from the above a-b-c formulae. Then we transform into an ellipse,
preserving $\Omega$. Suppose we then create a fresh RQS using the same
formulae based on $a$, $b$ and $c$ for the transformed positions. The
new RQS has the same path as the transformed one, albeit incorporating a
bilinear transformation.

In the figure this illustrates how this works when $\Omega=90^\circ$. We
can use the a-b-c formulae as one means to specify a quarter ellipse.
Also note the symmetric case of $b=c$. Overall, the a-b-c formulae
provide a convenient way to specify a “neutral ellipse” in that it has a
low eccentricity for a given case.

# Topic: Practical method for finding polynomials

The scheme for specifying an RQS in terms of control points $p_1$ and
$p_2$ is often convenient, but we need to be able to convert to a
canonical form. The form that we adopt is the one with $p_m$, $w_A$,
$w_B$ and $w_C$, modified such that if $w_B$ is zero and $p_m$
infinitely far away the numerator term $w_Bp_m$ can be found correctly.

Our approach is to find a vector $d$ in the direction $p_0$ to $p_3$ and
then find a vector $n$ by rotating $d$ through $90^\circ$
(anti-clockwise, by arbitrary choice).

$$
d = p_3-p_0 \label{Eq19}
$$

$$
n\cdot d = 0 \label{Eq20}
$$

Let $$ a = p_1-p_0 = \lambda(p_m-p_0) \label{Eq21} $$

$$
b = p_2-p_3 = \lambda(p_m-p_3) \label{Eq22}
$$

Hence

$$
d = \frac{a}{\lambda} - \frac{b}{\mu} \label{Eq23}
$$

The dot products of $d$ with $d$ and $n$ have simple results.

$$
\frac{a\cdot n}{\lambda} - \frac{b\cdot n}{\mu} = 0 \label{Eq24}
$$

$$
\frac{a\cdot d}{\lambda} - \frac{b\cdot d}{\mu} = \left| d\right|^2 \label{Eq25}
$$

Suppose that $\gamma$ satisfies

$$
\lambda = \frac{\gamma\left| d\right|^2}{b\cdot n} \label{Eq26}
$$

$$
\lambda = \frac{\gamma\left| d\right|^2}{a\cdot n} \label{Eq27}
$$

then the first dot product is satisfied. For the second

$$
\gamma = \frac{(a\cdot d)(b\cdot n)-(b\cdot d)(a\cdot n)}{\left| d\right|^4} \label{Eq28}
$$

Hence we can find weights for the RQS to satisfy earlier
conditionsequations and )}.

$$
w_A = \frac{3(b\cdot n)}{2\left| d\right|^2} \label{Eq29}
$$

$$
w_B = \left(\frac{3}{2}\right)^2\gamma \label{Eq30}
$$

$$
w_C = \frac{3(a\cdot n)}{2\left| d\right|^2} \label{Eq31}
$$

## Numerical generality

Two conditions must be met for a RQS to be valid.

$$
w_A \neq 0 \label{Eq32}
$$

$$
w_C \neq 0 \label{Eq33}
$$

These are required so that the path begins at $p_0$ and reaches $p_3$.
More tricky is the non-intersection cases. The case when $a$ and $b$ are
parallel is difficult to resolve, and we choose to declare that invalid.
In other words

$$
a\cdot n \neq 0 \label{Eq59}
$$

$$
b\cdot n \neq 0 \label{Eq60}
$$

Even when these conditions are met there may be no intersecton because
$a$ and $b$ can be parallel, meaning that $\gamma=0$ and $p_m$is
effectively off at infinity. We can readily resolve this by writing
$p_m$ in terms of $p_0$ or $p_3$ plus a displacement.

$$
p_n = w_Bp_m =
\left(\frac32\right)^2\gamma\left(
p_0 + \left(\frac{b\cdot n}{\gamma \left| d\right|^2}\right)(p_1-p_0)
\right) \label{Eq61}
$$

$$
p_n =
\left(\frac32\right)^2\left(
\gamma\, p_0 + \left(\frac{b\cdot n}{\left| d\right|^2}\right) a
\right) \label{Eq62}
$$

$$
p_n =
\left(\frac32\right)^2\left(
\gamma\, p_3 + \left(\frac{a\cdot n}{\left| d\right|^2}\right) b
\right) \label{Eq63}
$$

We could find $w_Bp_m$ as a weighted sum of these expressions, and in
fact one such weighting would effectively be like finding the
intersection $p_m$ explicitly, and therefore not work when $w_B=0$.
Overall, we recommend just picking one and using it. If some sort of
universal numerical consistency is desired, then a 50-50 combination is
good.

# Topic: Converting and matching

## Use cases

In this section we consider some conversion and matching methods that
may be useful when rendering curves. For instance, an SVG output API
might support elliptical arcs but not conic sections generally.
Furthermore, in such a case we will need to find a matching parabola
when the eccentricity of an ellipse is extreme such that the centre is
well off the page.

The design of Zebraix should account for the substitution of curves. For
example, suppose that an arrow attaches somewhere along the path of a
curve. We should delay extraction of the path location until late, and
preferrably after conversion. That assumes that we prefer to move the
arrow endpoint slightly and attach exactly to the curve, rather than
keeping the endpoint while knowing that the curve may be very slightly
off. Probably the best approach would be to do two passes, the first
deciding on conversion structural changes, and the second refining or
updating as needed. It should be noted that Zebraix constructs a DAG of
drawn entities. Therefore it should not be necessary to do two passes.
Note that the final drawing entities may have a rendering order
(“layers”) that does not need to correspond to any topological sort of
the calculation graph.

## Denominator balancing

Let us revisit the bilinear transformation, which enables us to adjust
the velocities at either end of an RQS while keeping the product of
those velocities constant. If we apply a transformation with

$$
\sigma = \sqrt\frac{w_A}{w_C} \label{Eq67}
$$

then, after that transformation, the coefficients of $t{}^2$ and
$(1-t){}^2$ in the denominator will be equal. In other words, $w_A=w_C$.
We refer to this as *denominator balancing*. The velocities will be
equal iff

$$
\left|p_m-p_3\right| = \left|p_m-p_0\right| \label{Eq68}
$$

### Scope of ellipse conversion

Rendering libraries like Cairo may not directly support RQSs, so we
convert to elliptical arcs. Hyperbolic conic sections may not be
supported. For this reason, for the present, Zebraix restricts the type
of RQS accordingly. One option is to divide such curves into pieces
approximated by cubic splines. However, we want to be cautious about
adding capabilities to Zebraix that might either complicate code or
significantly increase the size of output files.

When a RQS is close to a limitation, such as when a parabola is
specified, it is possible that numerical realities cause the conditions
to go over the boundary. Zebraix incorporates some tolerance for this,
and will “snap” the RQS to a parabola.

### Ellipse conversion method

<figure>
<img src="figs-ratquad/RatQuad-G.svg"
alt="Creating an elliptical arc by affine transformation of a circular arc. The centre is transformed to p_a, the point (1,0) is transformed to p_a+p_s, and the point (0,1) is transformed to p_a+p_s." />
<figcaption aria-hidden="true">Creating an elliptical arc by affine
transformation of a circular arc. The centre is transformed to <span
class="math inline"><em>p</em><sub><em>a</em></sub></span>, the point
<span class="math inline">(1,0)</span> is transformed to <span
class="math inline"><em>p</em><sub><em>a</em></sub> + <em>p</em><sub><em>s</em></sub></span>,
and the point <span class="math inline">(0,1)</span> is transformed to
<span
class="math inline"><em>p</em><sub><em>a</em></sub> + <em>p</em><sub><em>s</em></sub></span>.</figcaption>
</figure>

The overall flow of the conversion from RQS to ellipse arc is as
follows.

1.  First, balance the denominator of the RQS.
2.  Normalize by the sign of $w_A$.
3.  Shift the range of $t$ so that the RQS is over the range
    $[-1/2, 1/2]$.

For these first $3$ steps let

$$
\rho = \frac{w_B}{\sqrt{w_Aw_C}} \label{Eq69}
$$

$$
p_q = \rho p_m \label{Eq70}
$$

$$
p_q = \left(\frac{1}{\sqrt{w_Aw_C}}\right) p_n \label{Eq71}
$$

$$
p_q = \left(\frac23\right)\frac{\left|d\right|^2}{\sqrt{(a\cdot n)(b\cdot n)}} \label{Eq72}
$$

Then

$$
f_\rho(t) = \frac{%
\Bigl(\frac12-t\Bigr)^2p_0 + 2\Bigl(\frac12-t\Bigr)\Bigl(\frac12+t\Bigr)p_q + \Bigl(\frac12+t\Bigr)^2p_3
}{%
\Bigl(\frac12-t\Bigr)^2 + 2\rho\Bigl(\frac12-t\Bigr)\Bigl(\frac12+t\Bigr) + \Bigl(\frac12+t\Bigr)^2
} \label{Eq73}
$$

1.  (Step 4) Express the denominator in terms of $\alpha$ and $\beta$
    such that

$$
\alpha = \frac12(1+\rho) \label{Eq74}
$$

$$
\alpha\beta^2 = \frac12(1-\rho) \label{Eq75}
$$

$$
f_\rho(t) = \frac{%
\frac14\bigl(p_0+2p_q+p_3) + t\bigl(p_3-p_0\bigr) + t^2\bigl(p_o-2p_q+p_3\bigr)
}{%
\alpha(1+\beta^2t^2)
} \label{Eq76}
$$

1.  (Step 5) Split the spline path into trigonometric parts. That is,
    let

$$
p_a = \frac1{8\alpha}\Bigl(p_0+2p_q+p_3\Bigr) + \frac1{2\alpha\beta^2}\Bigl(p_0-2p_q+p_3\Bigr) \label{Eq77}
$$

$$
p_c = \frac1{8\alpha}\Bigl(p_0+2p_q+p_3\Bigr) - \frac1{2\alpha\beta^2}\Bigl(p_0-2p_q+p_3\Bigr) \label{Eq78}
$$

$$
p_s(t) = \frac1{2\alpha\beta}\Bigl(p_3-p_0\Bigr) \label{Eq79}
$$

$$
f_\rho(t) = p_a + \left(\frac{2\beta t}{1+\beta^2 t^2}\right)p_s + \left(\frac{1-\beta^2 t^2}{1+\beta^2 t^2}\right)p_c \label{Eq80}
$$

Since the range of $t$ here is $-1/2$ to $1/2$, and this is tan of the
half angle, the angular range for the arc is

$$
\left[\, -2\tan^{-1}\left(\frac\beta2\right),\,\, 2\tan^{-1}\left(\frac\beta2\right)\, \right] \label{Eq81}
$$

Once we have this, we create, as shown, a circular arc radius $1$with
the required angle range. This is then transformed, moving the centre to
$p_a$ and the mid-point to $p_a+p_c$. The point $(0,1)$ may or may not
be part of the unit circle arc, and is in either case transformed to
$p_a+p_s$.

## Constraints and conversion to parabolas

We start by restricting

$$
(a\cdot n)(b\cdot n) > 0 \label{Eq82}
$$

The rationale is that an elliptical arc can always be an ellipse cut by
a straight line. The ellipse can be stretched into a circle without
affecting the signs of $a\cdot n$ and $b\cdot n$, which are necessarily
the same for a line cutting a circle.

This means that

$$
\lambda\mu > 0 \label{Eq83}
$$

$$
w_Aw_C > 0 \label{Eq84}
$$

In step 2 above we normalized the RQS to have $w_A>0$, so this implies
$w_C>0$ thereafter. If

$$
\frac{(a\cdot n)(b\cdot n)}{\left|d\right|^2} \label{Eq85}
$$

is smaller than a threshold, we reject. This is not identifiably of any
particular type of conic section, a matter we will address shortly.

<!-- mdformat off (Document metadata) -->

| Arc type                    | Angle range                      | $\beta$      | $\rho$   | $\alpha$  |
|:----------------------------|:---------------------------------|:-------------|:---------|:----------|
| Parabola                    | $-\epsilon^\circ,\epsilon^\circ$ | $0$          | $1$      | $1$       |
| Arc less than semicircle    |                                  | $(0,2)$      | $(0,1)$  | $(1/2,1)$ |
| Semicircle                  | $-90^\circ,90^\circ$             | $2$          | $0$      | $1/2$     |
| Arc greater than semicircle |                                  | $(2,\infty)$ | $(-1,0)$ | $(0,1/2)$ |
| Full circle / indeterminate | $-180^\circ,180^\circ$           | $\infty$     | $-1$     | $0$       |

Ranges of $\beta$, $\rho$ and $\alpha$ for ranges of RQS with
trigonometric curves. An arc of small angle ($\epsilon$ here), and very
large scaling becomes, in the limit, a parabola. The range of $t$ is
$[-1/2,1/2]$. Outside of these ranges the curve is a hyperbola. Zebraix
does not support these.

<!-- mdformat on -->

Otherwise, that is if the value is above the threshold, then $\rho$ is
well-determined. We then need to consider $\alpha$ and $\beta$. It works
best to consider which values lead to which ranges of arc angles. These
are set out in the table. Zebraix only supports elliptical-arc and
parabolic RQSs, and therefore

$$
-1 < \rho < 1 \label{Eq86}
$$

for elliptical arcs. The condition for parabolas is that $\rho$ is close
to $1$.

To convert to parabolas we basically set $\rho=1$ and have $\beta=0$ in
the denominator. By doing this the denominator of $f_\rho(t)$ loses the
$t{}^2$ term. We can implement this, in the original expression for
$f(t)$, by setting $w_B$ according to

$$
w_B = \sqrt{w_Aw_C} \label{Eq87}
$$

That is,

$$
w_B = \left(\frac32\right) \frac{\sqrt{(a\cdot n)(b\cdot n)}}{\left|d\right|^2} \label{Eq88}
$$

The adjusted curve is a bilinear transformation of a parabola/

1.  When extracting points we use the adjusted RQS.

2.  When drawing the spline path, we convert to a cubic spline using
    $f_\rho(t)$. Let this be $f_\alpha(t)$ for $0\leq t\leq 1$.

$$
f_\alpha(t) = (1-t)^3p_0 + 3t(1-t)^2\left(\frac23p_q + \frac13p_0\right) + 3t^2(1-t)\left(\frac23p_q + \frac13p_3\right) + t^3p_3 \label{Eq89}
$$

If, on the other hand, $\rho$ is close to $-1$ we reject the RQS as
invalid: it is hyperbolic, or too close to being so.

## Intersection-angle form

If an RQS is specified in intersection-angle form ($p_m$, $\Omega$,
$\sigma$), we can convert to weighted form as follows.

$$
\lambda = \sigma\sqrt{\frac29\Bigl(1-\cos(\Omega)\Bigr)} = \frac{2w_B}{3w_A} \label{Eq90}
$$

$$
\mu = \frac1\sigma\sqrt{\frac29\Bigl(1-\cos(\Omega)\Bigr)} = \frac{2w_B}{3w_C} \label{Eq91}
$$

Let

$$
w_A = \frac1\sigma \label{Eq92}
$$

$$
w_B = \sqrt{\frac{1-\cos(\Omega)}{2}} = \rho \label{Eq93}
$$

$$
w_C = \sigma \label{Eq94}
$$

The special case $\Omega=0$ corresponds to the semi-circle. In this case
the elliptical arc is squished down to a line, with $p_q=0$. It is an
arc that can be represented as a transformed circle. On the other hand,
$\Omega$ near $180^\circ$ corresponds to a small arc angle, and thus one
to be converted to a parabola.

## Summary

We typically cannot use RQSs directly in many key cases, including
rendering to SVGs. Therefore we convert RQSs using 4 forms.

1.  The form of RQS in which it is specified. This might be the
    control-points form or the intersection-angle form.

2.  The RQS converted to ($W_A$, $w_B$, $w_C$, $p_m$)-form: weights
    form.

3.  Adjusted form. This is the weights form, but when appropriate with
    the denominator adjusted to bilinear-parabola. If this adjustment is
    made, the path may change slightly. In that use, the spline is not a
    parabolic spline, but a parabola with bilinear transformation.
    Therefore points extracted from the adjusted RQS will be close to
    those originally specifid.

4.  Path form, either: (a) Elliptical arc in ($\beta$, $p_a$, $p_s$,
    $p_c$)-form, or (b) Parabolic spline in $f_\alpha(t)$ form.

In addition to the above, parabolas may be used in colinear cases. This
must be via the intersection-angle specification.

# Topic: Choice of specification schemes

## Requirements

We could not find a single scheme for specifying RQSs. Basically the
difficulty is in the range of angles. We want to be able to specify
semi-circles and to be able to specify colinear RQSs. We want to be able
to specify cases near these with convenience and accuracy.

It is also important that the specification of the curves be invariant
on transformation. This means that we get the same RQS if (a) we find a
curve path, or points along one, and then apply a transformation, or (b)
we transform the specification and find the curve path.

As a result we provide more than one specification scheme. We want to
avoid proliferation of schemes, and want to aim for stability of the
user interface. Multiple extensions or changes later will not
necessarily complicate Zebraix’s core code very much, but will increase
testing and user documentation. Later deprecation would necessitate
migration of drawing libraries.

## Specification schemes

Zebraix supports 2 specification schemes for RQSs.

- 1.  A *four point*, or *control-point*, scheme: ($p_0$, $p_1$, $p_2$,
      $p_3$).

  - 1)  When a Bézier cubic spline is a parabola, the four points used
        for an RQS also generate a parabola.

  - 2)  An intersection point is not required, so semi-circles can be
        specified.

  - 3)  Intersection points that are colinear with the end points are
        not supported.

- 2.  *Intersection-angle* scheme: ($p_0$, $p_m$, $p_3$, $\cos(\Omega)$,
      $\sigma$).

  - 1)  Intersection points may be colinear with their end points.

  - 2)  Semi-circles cannot be specified.

  This scheme is the one based on $\Omega$, parameterized by
  $\cos(\Omega)$. A bilinear transformation on $\sigma$ is incorporated,
  speeding up the velocity by $\sigma$ at $p_0$ and slowing it down by
  $\sigma$ at $p_3$.

In addition to RQSs, Zebraix supports cubic splines and straight lines.
Straight lines can have bilinear transformation to adjust end-point
velocities. Zebraix also supports complete circles and ellipses. It also
provides a most-circular version of the intersection-angle scheme in
which $\Omega$ is set automatically for the ease of rounding corners.
(This must be calculable before transformation.)

<figure>
<img src="figs-ratquad/RatQuad-I.svg"
alt="RQSs with \cos(\Omega)=0 and \sigma=1 can be used conveniently for purposes such as the rounded corners of a parallelogram. The drawing shows such a parallelogram (top) along with a breakdown highlighting the corners. This illustrates how the mid-point with respect to t, that is t=1/2 within the range [0,1], divides the corner RQS at its logical mid-point. This is where the parallelogram diagonal intersects the RQS." />
<figcaption aria-hidden="true">RQSs with <span
class="math inline">cos (<em>Ω</em>) = 0</span> and <span
class="math inline"><em>σ</em> = 1</span> can be used conveniently for
purposes such as the rounded corners of a parallelogram. The drawing
shows such a parallelogram (top) along with a breakdown highlighting the
corners. This illustrates how the mid-point with respect to <span
class="math inline"><em>t</em></span>, that is <span
class="math inline"><em>t</em> = 1/2</span> within the range <span
class="math inline">[0,1]</span>, divides the corner RQS at its logical
mid-point. This is where the parallelogram diagonal intersects the
RQS.</figcaption>
</figure>

## Convenience features of intersection-angle scheme

A parallelogram is a transformed square. Therefore, we can conveniently
specify rounded corners of a parallelogram with $\cos(\Omega)=0$,
because the corners are similarly transformed quarter circles. This is
illustrated.

Another convenience is that the scheme maintains symmetry. That is to
saym, if the RQS is symmetric and we choose $\sigma=1$, then our
formulation retains that symmetry. For instance, it means that we can
substitute $1-t$ for $t$ and exchange $p_0$ and $p_3$. This is possible
because, in the symmetric case, the displacement of $p_m$ from the
centre (or just $p_m$?) is proportional to $(p_0+p_3)$. This has two
consequences that we note here.

1.  If we extract points from the RQS and if symmetry is retained, then
    the points are logically symmetric. If we round the corners of a
    square or rhombus, they are literally symmetric. (The parallelogram
    is logically a stretched version of the literal case.)

2.  The mid-point is at $t=1/2$. For example, in the parallelogram
    example this is the intersection of the diagonal and the RQS.

# Topic: Point distribution, parameterization and attachment

<figure>
<img src="figs-ratquad/RatQuad-J.svg"
alt="Example vertegrams with angular attachment (left column) and parallel attachment (right column). All vertegrams should work with either spacing. Circular vertegrams lend themselves best to angular spacing and flat faces lend themselves most readily to parallel spacing. All vertegrams should have continuous attachments, all the way around, including sharp or rounded corners. Vertegrams have four sides, which are intended for parallel attachment. Vertegrams must support both styles of attachment, even if much better suited to one versus the other." />
<figcaption aria-hidden="true">Example vertegrams with angular
attachment (left column) and parallel attachment (right column). All
vertegrams should work with either spacing. Circular vertegrams lend
themselves best to angular spacing and flat faces lend themselves most
readily to parallel spacing. All vertegrams should have continuous
attachments, all the way around, including sharp or rounded corners.
Vertegrams have four sides, which are intended for parallel attachment.
Vertegrams must support both styles of attachment, even if much better
suited to one versus the other.</figcaption>
</figure>

<figure>
<img src="figs-ratquad/RatQuad-K.svg"
alt="Attachment points, whether parallel or angular, are parameterized counter-clockwise from the mid-line of each side." />
<figcaption aria-hidden="true">Attachment points, whether parallel or
angular, are parameterized counter-clockwise from the mid-line of each
side.</figcaption>
</figure>

<!-- mdformat off (Document metadata) -->

| Vertegram face style | $\tau=$                      | $\tau=$                                |
|:---------------------|:-----------------------------|:---------------------------------------|
| Linear-tangent       | $u$                          | $\tan(\phi)$                           |
| Circular-tan-half    | $\tan(\tfrac12\tan^{-1}(u))$ | $\tan(\phi/2)$                         |
| Sin-tan-half         | $\tan(\tfrac12\sin^{-1}(u))$ | $\tan(\tfrac12\sin^{-1}(u\tan(\phi)))$ |

Conversion formulae that Zebraix uses internally to take parallel or
angular attachment parameter values to those preferred by a vertegram
face. Scaling factors may be incorporated. Linear-tangent style converts
from an angular vertegram approach ($\phi$) to parallel spacing. A
*circular-tan-half* style of face needs the linear or angular approach
to be converted to a tan-half-angle. Zebraix may also support face
attachment like *sin-tan-half*, which in essence converts an angular
vertegram approach to even parallel spacing.

<!-- mdformat on -->

Zebraix calls the basic vertex drawable entitly a *vertegram*. A
vertegram takes the form of a circle, box, logic gate or network node,
and has a flow direction. This may be somewhat arbitrary, such as for a
circle. For a logic gate the direction is natural. We enumerate various
features of vertegrams, each with implied design decisions. The
illustration depict these( see figures and ).

1.  Each vertex has four faces: *inbound*, *outbound*, *lefthand* and
    *righthand*.

2.  The direction can be especially arbitrary for, say, a storage drive,
    in which case we use a left-right flow. Vertegrams may not rotate
    with the flow. In contrast to this, logic gate would be rotated if
    in a top-to-bottom flow.

3.  There are two styles of attachment: *angular* and *parallel*. These
    reflect the kind of connection pattenrs. In pure form, jalwalks are
    drawn as simple graphs with straight edges and circular vertegrams.
    This is angular attachment. A family tree or similar style of
    drawing have connections that approach their vertegrams in parallel,
    sometimes with perpendicular side inputs.

4.  Vertegrams are typically designed with one attachment style in mind,
    and Zebraix supports variations in vertegram specification to
    support this.

5.  Vertegrams are required, even if they prefer parallel attachment, to
    provide a continuous attachment “perimeter” divided into the four
    faces.

6.  Each face has a connected spline sequence. For example, the input
    face of a rounded rectangle would be 1/8 of a circle connected to a
    straight line connected to 1/8 of a circle.

7.  The drawing specification and attachment specification of a
    vertegram are separate, but for convenience the attachment can
    reference coordinates, and so on, extracted from the drawing.

8.  There may be alternative attachment specifications, which are more
    forgiving about discontinuities. For example, both the drawing and
    attachments may change for a logic gate if there are a large number
    of inputs.

9.  Locations are parameterized in the range $[-1/2, 1/2]$, with $0$
    ordinarily connecting at the centreline. This contrasts by offset
    with spline components, which are parameterized in the more
    traditional $[0,1]$ range.

10. Spline ranges can be subdivided. Offsets and scale factors are
    incorporated as approriate. For example, the input face of a rounded
    rectangle can take subdivisions, specifically half, of the top-left
    and bottom-left quarter-circle rounded corners. Thus the attachment
    faces would not specify their own splines, but select, subdivide,
    and chain those from the vertegram drawing.

11. The subdivision of the parameter can be used to avoid, or at least
    discourage, attachment to specific splines. For example, we may wish
    to discourage attachment to rounded corners. Parallel attachment is
    guaranteed to be in the range 5%-95%, and strongly avoids outside of
    10%-90% of the range. If there are lots of inputs they may extend
    into the wider range. Discouragement is specified through weights
    that reduce the parameter range associated with a particular spline.
    This should not be too strong, as otherwise angular attachment could
    be undesirably jumpy. This mechanism is called *segment weighting*.

12. Vertegrams have input-output and left-right centrelines, and
    vertegram centres, here *anchor points*, are the intersections of
    these two lines. Ordinarily the zero-parameter attachment locations
    on faces coincide with the intersections of faces and centrelines.
    If vertegrams are aligned by centreline they should look correct.
    For example, logic gates should be correctly aligned avertically by
    left-right centrelines

13. Zebraix does not actually attach angular connections with strict
    correctness. Basically, in the untransformed graph the
    centre-to-centre lines between vertices are measured against
    horizontal. This is then fixed. So, for example, a rectangular
    vertegram will attach $45^\circ$ lines at its corners. The resulting
    edge lines, when extrapolated, will not pass through vertegram
    centres. The rationale for this is that Zebraix should be
    “logically” or “semantically” consistent. We are less concerned with
    exact geometry.

14. Often we want even attachment for a spline that does not have even
    displacement. In contrast to a parabola, which we can fix to have
    uniform attachment, a circle would not by angle space evenly.
    Equivalently, a straight edge would not attach evenly with respect
    to angle. The table shows some candidate conversions that Zebraix
    may use in conjunction with splines in order to achieve even
    parallel or angular attachment.

# Other design choices and considerations

## Stretch factors

Zebraix has a mix of affine transformation factors.

- *Literal scale* is a special scaling factor applied at the end that is
  a pure scale, auch as for different screen resolutions. This should
  only be used rarely. Zebraix prefers to output vector graphics and let
  the final renderer deal with resolutions and the like.

- *General scale factors* modify how the vertices are spaced, and how
  the drawing is laid out. All features of the graph can have affine
  transformations based on these. There are separate horizontal and
  vertical stretches, which are combined with a global scale. A third
  factor is created, an amalgamated global scale, which is the geometric
  mean of the horizontal and vertical (or the square root of an affine
  determinant). The rationale behind the extra factor is that, for
  example, a circular vertegram can be isotropically scaled instead of
  being turned into an ellipse. We still want to be able to stretch the
  layout more in one direction than another. General drawing scale
  factors are rich enough that they scale different aspects. The idea is
  that we want to be able to draw a jaywalk with 20 nodes or 4 nodes on
  a page. The one with 20 nodes should not have tiny vertegrams. There
  should be sizing coming from other factors, which appears to general
  affine transformation as an offset.

- *Content scale factors* are specifically linked to line thickness and
  font size. Initially we will keep to a single content scale factor,
  and scale line thicknesses and font sizes from this, or vice versa. If
  we use a base line thickness of 2pt, we want this to rise to 3pt when
  drawing in “heavy” weight and 1.5pt when drawing in “light” weight.
  The font size might be 12pt for medium weight, 14 for heavy, 10 for
  light. The idea here is that the margin around text in a box should
  increase with the drawing weight. Arrow sizes should be in an affine
  relation with the weight and/or line thickness.
