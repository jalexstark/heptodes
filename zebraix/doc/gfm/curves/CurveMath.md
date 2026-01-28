---
author:
- J. Alex Stark
date: 2025
title: A Compendium of Curve Mathematics in Zebraix
---

- [<span class="toc-section-number">1</span> Setting the
  scene](#setting-the-scene)
  - [<span class="toc-section-number">1.1</span> Traditional
    curves](#traditional-curves)
  - [<span class="toc-section-number">1.2</span> Zebraix
    distinctives](#zebraix-distinctives)
  - [<span class="toc-section-number">1.3</span> Attachment and bilinear
    warping](#attachment-and-bilinear-warping)
- [<span class="toc-section-number">2</span> Weight and power, rational
  quad and cubilinear](#weight-and-power-rational-quad-and-cubilinear)
  - [<span class="toc-section-number">2.1</span> Bilinear
    transformation](#bilinear-transformation)
  - [<span class="toc-section-number">2.2</span> Expressing bilinear
    transformations in
    curves](#expressing-bilinear-transformations-in-curves)
  - [<span class="toc-section-number">2.3</span> Cubilinear curves
    (CLCs)](#cubilinear-curves-clcs)
  - [<span class="toc-section-number">2.4</span> Matrix
    conversions](#matrix-conversions)
- [<span class="toc-section-number">3</span> Slicing and
  splicing](#slicing-and-splicing)
  - [<span class="toc-section-number">3.1</span>
    Derivatives](#derivatives)
  - [<span class="toc-section-number">3.2</span> Cubic](#cubic)
  - [<span class="toc-section-number">3.3</span> Velocities and
    end-point derivatives](#velocities-and-end-point-derivatives)
    - [<span class="toc-section-number">3.3.1</span> Cubic
      end-points](#cubic-end-points)
    - [<span class="toc-section-number">3.3.2</span> RQ
      end-points](#rq-end-points)
  - [<span class="toc-section-number">3.4</span> RQC cuts](#rqc-cuts)
    - [<span class="toc-section-number">3.4.1</span> Bilinear
      cuts](#bilinear-cuts)
    - [<span class="toc-section-number">3.4.2</span> Making
      cuts](#making-cuts)
  - [<span class="toc-section-number">3.5</span> CLC cuts](#clc-cuts)
  - [<span class="toc-section-number">3.6</span> Range
    reassignment](#range-reassignment)
  - [<span class="toc-section-number">3.7</span> Splicing](#splicing)
- [<span class="toc-section-number">4</span> Curve
  specification](#curve-specification)
  - [<span class="toc-section-number">4.1</span> Canonical
    quarter-ellipse and
    variants](#canonical-quarter-ellipse-and-variants)
  - [<span class="toc-section-number">4.2</span> RGC
    four-point](#rgc-four-point)
  - [<span class="toc-section-number">4.3</span> Three-point
    RGC](#three-point-rgc)
  - [<span class="toc-section-number">4.4</span> Three-point RGC
    calculation](#three-point-rgc-calculation)
  - [<span class="toc-section-number">4.5</span> Cutting
    corners](#cutting-corners)
  - [<span class="toc-section-number">4.6</span>
    Cubilinear](#cubilinear)
  - [<span class="toc-section-number">4.7</span> Straight
    lines](#straight-lines)
- [<span class="toc-section-number">5</span> Elaborated
  topics](#elaborated-topics)
  - [<span class="toc-section-number">5.1</span> Matching
    velocities](#matching-velocities)
- [<span class="toc-section-number">6</span> Other (maybe) handy
  stuff](#other-maybe-handy-stuff)
  - [<span class="toc-section-number">6.1</span> Bilinear
    transformation](#bilinear-transformation-1)
  - [<span class="toc-section-number">6.2</span> Traditional power forms
    and derivatives](#traditional-power-forms-and-derivatives)
  - [<span class="toc-section-number">6.3</span> Degree-1 matrix
    conversions](#degree-1-matrix-conversions)
  - [<span class="toc-section-number">6.4</span> Range cut](#range-cut)
  - [<span class="toc-section-number">6.5</span> Derivative
    bits](#derivative-bits)
  - [<span class="toc-section-number">6.6</span> Cutting
    corners](#cutting-corners-1)
  - [<span class="toc-section-number">6.7</span> Cubilinear curve
    specification](#cubilinear-curve-specification)
  - [<span class="toc-section-number">6.8</span> RQC solvable
    split](#rqc-solvable-split)

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
<!-- ================================================================== -->

# Setting the scene

## Traditional curves

$$
g(t) = (1-t)^3p_0 + 3t(1-t)^2p_1 + 3t^2(1-t)p_2 + t^3p_3 \text{.}\label{Eq01}
$$

For RQCs, we use a three-point expression along with three weights
($w_A$, $w_B$, $w_C$).

$$
f(t) = \frac{%
w_A(1-t)^2p_0 + 2w_Bt(1-t)p_m + w_Ct^2p_3
}{%
w_A(1-t)^2 + 2w_Bt(1-t) + w_Ct^2
} \label{Eq03}
$$

## Zebraix distinctives

- We care about exact ellipse drawing. Other applications of curves
  would be fine approximating using polynomial curves.
- Trad is $t \in [0,1]$, whereas we generalize to $t \in [v,w]$.
- Curves written both weighted and power forms, we use weighted
  internally for the most part.
- Variety of specification methods, for these preferring affine
  invariance. Example of corners of a parallelogram being, in some
  senses “logically” right angles even though “physically” not.
- We also care about solvability. That is, for some curve specifications
  it is possible first to set some parameters and then set constrainable
  locations and lengths.

$$
\begin{aligned}
x(t)
&= \frac{b(t)}{a(t)} \\
y(t)
&= \frac{c(t)}{a(t)}
\end{aligned}
$$ where, in *weighted* form, $$
b(t) = b_0(w-t)^2 + b_1(t-v)(w-t) + b_2(t-v)^2
$$ and so on. We also use *power* form in which $$
b(t) = b_0 + b_1 t + b_2 t^2
$$ and so on, but generally prefer to work with weighted form
internally. We chose the name “power” since the polynomials are
expressed in term of powers of $t$, whereas in the weighted form the
polynomials encode the end points. For example, $x(v) = b_0/a_0$.

(Aside: Those familiar with cubic splines will note that the weighted
form does not directly weight the traditional control points. We scale
the coefficients for convenience in internal computation.)

## Attachment and bilinear warping

Curves for box elements, and more complicated. Idea of ports for
attaching connections. “Procedural” approach, with continuous attachment
loops.

<!-- ================================================================== -->

# Weight and power, rational quad and cubilinear

## Bilinear transformation

Some touching on derivatives, because shows sigma factor.

General for range $[u, w]$.

$$
h(t; \sigma) = \frac{%
v(w -t) +  \sigma w(t-v)
}{%
(w -t) +  \sigma (t-v)
}
$$

<!-- $$ -->
<!-- h(t; \sigma) = \frac{% -->
<!-- (\sigma w -v)t - (\sigma-1) vw -->
<!-- }{% -->
<!-- (\sigma-1)t + (w - \sigma v) -->
<!-- } -->
<!-- $$ -->

Constraints. $$
\begin{aligned}
h(w; \sigma) &= w \\
h(v; \sigma) &= v \\
h(h(t; \sigma_1); \sigma_2) &= h(t; \sigma_1 \sigma_2) \\
h(h(t; \sigma); \sigma^{-1}) &= t
\end{aligned}
$$

Derivative.

From the chain rule

$$
\frac{\mathrm{d}x(h(t; \sigma))}{\mathrm{d}t}
=
x'(t)\, h'(t;\sigma)
\text{.}
$$

In other words, if we differentiate a curve path that has been modified
by bilinear transformation, the result is simply the product of the
unmodified differential and $h'(h; \sigma)$.

$$
\begin{aligned}
h'(t; \sigma) &= \frac{%
(w - v)^2 \sigma
}{%
% ((\sigma-1)t + (w - \sigma v))^2
((w -t) +  \sigma (t-v))^2
} \\
h'(w; \sigma) &= \frac{1}{\sigma} \\
h'(v; \sigma) &= \sigma
\end{aligned}
$$

In particular, the product of the accelerating effect at $t=v$ and the
decelerating effect at $t=w$ is $1$. It turns out that we see a lot of
$(w -t) + \sigma (t-v)$, which becomes $w-v$ when $\sigma =1$. Sometimes
it substitutes for $w-v$, and sometimes it appears as a ratio with
$w-v$.

## Expressing bilinear transformations in curves

An additional key fact is that $$
\frac{h(t; \sigma) - v}{w-h(t; \sigma)}
=
\frac{\sigma(t-v)}{w-t} \text{.}
$$

Consider $x(t)$, and its transformation into $x(h(t; \sigma))$.

$$
\begin{aligned}
\frac{b(t)}{a(t)}
&=
\frac{%
b_0(w-t)^2 + b_1(t-v)(w-t) + b_2(t-v)^2%
}{%
a_0(w-t)^2 + a_1(t-v)(w-t) + a_2(t-v)^2%%
}
\\
\frac{b(h(t; \sigma))}{a(h(t; \sigma))}
&=
\frac{%
b_0(w-t)^2 + \sigma b_1(t-v)(w-t) + \sigma^2 b_2(t-v)^2%
}{%
a_0(w-t)^2 + \sigma a_1(t-v)(w-t) + \sigma^2 a_2(t-v)^2%%
}
\end{aligned}
$$

Therefore we express RQCs in relatively simple weighted form,
incorporating $\sigma$.

$$
\begin{aligned}
b(t) &= b_0(w-t)^2 + \sigma b_1(t-v)(w-t) + \sigma^2 b_2(t-v)^2 \text{,}\\
c(t) &= c_0(w-t)^2 + \sigma c_1(t-v)(w-t) + \sigma^2 c_2(t-v)^2 \text{,}\\
a(t) &= a_0(w-t)^2 + \sigma a_1(t-v)(w-t) + \sigma^2 a_2(t-v)^2 \text{.}
\end{aligned}
$$

Note, however, that in implementations we may choose to normalize the
encodings of curves by a common factor.

Expressed more concisely using matrices,

$$
  \begin{bmatrix}
b(t) \\
c(t) \\
a(t)
  \end{bmatrix} =
  \begin{bmatrix}
  b_0 & b_1 & b_2 \\
  c_0 & c_1 & c_2 \\
  a_0 & a_1 & a_2
  \end{bmatrix}
  \begin{bmatrix}
    1 & 0 & 0 \\
    0 & \sigma & 0 \\
    0 & 0 &  \sigma^2
  \end{bmatrix}
  \begin{bmatrix}
  (w-t)^2 \\
  (w-t)(t-v) \\
  (t-v)^2
  \end{bmatrix}
  \text{.}
$$

## Cubilinear curves (CLCs)

In Zebraix we like consistency. More than that, we want the benefits of
bilinear transformation with cubic curves. RQCs inherently support
bilinear transformation. That is to say the capability is hidden within
the coefficients, and our preceding analysis really only exposed or
interpreted something already there. For cubic curves we actually invent
an extension, and we call these cubilinear curves (CLCs). Consider the
$x$ component of a cubic curve, and then combine it with a bilinear
transformation. In the first expression, the denominator is really just
$(w-v){}^3$, but we write it to more closely match the denominator of a
RQC.

$$
\begin{aligned}
x(t) &=
\frac{b_0(w-t)^3 +  b_1(t-v)(w-t)^2 + b_2(t-v)^2(w-t) + b_3(t-v)^3}%
{((w-t) + (t-v))^3} \text{;}\\
x(h(t; \sigma)) &=
\frac{b_0(w-t)^3 + \sigma b_1(t-v)(w-t)^2 + \sigma^2 b_2(t-v)^2(w-t) + \sigma^3 b_3(t-v)^3}%
{((w-t) + \sigma(t-v))^3} \text{.}
\end{aligned}
$$

Let us write our CLC path in a manner similar to RQC, and summarize
using matrices.

$$
  \begin{bmatrix}
x(t) \\
y(t)
  \end{bmatrix} =
  \begin{bmatrix}
b(t)/a(t) \\
c(t)/a(t)
  \end{bmatrix}
$$

where

$$
  \begin{bmatrix}
b(t) \\
c(t) \\
a(t)
  \end{bmatrix} =
  \begin{bmatrix}
  b_0 & b_1 & b_2 & b_3 \\
  c_0 & c_1 & c_2 & c_3 \\
    1 & 3 & 3 & 1
  \end{bmatrix}
  \begin{bmatrix}
    1 & 0 & 0 & 0 \\
    0 & \sigma & 0 & 0 \\
    0 & 0 &  \sigma^2  & 0 \\
     0 & 0 & 0 &   \sigma ^3
  \end{bmatrix}
  \begin{bmatrix}
  (w-t)^3 \\
  (w-t)^2(t-v) \\
  (w-t)(t-v)^2 \\
  (t-v)^3
  \end{bmatrix}
$$

When representing CLCs within code, we cannot just move the $\sigma$
factor into the $b_0$, etc, coefficients. We would have to have a full
representation for $a(t)$, but that would introduce redundancy and
complicate methods that manipulate these curves. In practice it is
fairly straightforward to work with $\sigma$ as a separate factor. This
is a key reason why we like to work internally with weighted form rather
than power form.

## Matrix conversions

The matrix expression of RQC in power form is simply

$$
  \begin{bmatrix}
  b(t) \\
  c(t) \\
  a(t)
  \end{bmatrix} =
  \begin{bmatrix}
  b_0 & b_1 & b_2 \\
  c_0 & c_1 & c_2 \\
  a_0 & a_1 & a_2
  \end{bmatrix}
%  \begin{bmatrix}
%  1 & 0 & 0 \\
%  0 & \sigma & 0 \\
%  0 & 0 &  \sigma^2
%  \end{bmatrix}\
\begin{bmatrix}
  1 \\
  t \\
  t^2
  \end{bmatrix}
  \text{.}
$$

The bilinear transform is not readily expressed, so we need to
incorporate that into the coefficients $a_0$ through $c_2$.

We can convert to power form, and back to weighted form, using

$$
\begin{aligned}
  \begin{bmatrix}
  (w-t)^2 \\
  (w-t)(t-v) \\
  (t-v)^2
  \end{bmatrix}
  &=
\mathbf{R}_2(w,v)\,
  \begin{bmatrix}
  1 \\
  t \\
  t^2
  \end{bmatrix}
  \text{,}
  \\
  \begin{bmatrix}
  1 \\
  t \\
  t^2
  \end{bmatrix}
  &=
\mathbf{S}_2(w,v)\,
  \begin{bmatrix}
  (w-t)^2 \\
  (w-t)(t-v) \\
  (t-v)^2
  \end{bmatrix}
  \text{.}
\end{aligned}
$$

The $\mathbf{R}_2(w,v)$, its inverse $\mathbf{S}_2(w,v)$, and the
equivalents for cubic curves, are

$$
\begin{aligned}
\mathbf{R}_2(w,v)
  &=
\begin{bmatrix}
w^2 & -2w & 1\\
-vw & v+w & -1\\
v^2 & -2v & 1
\end{bmatrix}
  \text{,}
  \\
\mathbf{S}_2(w,v)
  &=
  \frac{1}{(w-v)^2}
\begin{bmatrix}
1 & 2 & 1 \\
v & v+w & w \\
v^2 & 2vw & w^2
\end{bmatrix}
  \text{,}
\\
\mathbf{R}_3(w,v)
  &=
\begin{bmatrix}
w^3 &  -3w^2 & 3w & -1\\
-vw^2 & w(2v+w) & -(v+2w) & 1\\
v^2w & -v(v+2w) & (2v+w) & -1\\
-v^3 &  3v^2 & -3v & 1
\end{bmatrix}
  \text{,}
  \\
\mathbf{S}_3(w,v)
  &=
  \frac{1}{(w-v)^3}
\begin{bmatrix}
1 &  3 & 3 & 1\\
v & 2v+w & v+2w & w\\
v^2 & v(v+2w) & w(2v+w) & w^2\\
v^3 &  3v^2w & 3vw^2 & w^3
\end{bmatrix}
  \text{.}
\end{aligned}
$$

<!-- ================================================================== -->

# Slicing and splicing

## Derivatives

At the risk of abusing notation, let

$$
b^*(t) =
a(t)\frac{\mathrm{d}b(t)}{\mathrm{d}t} - b(t)\frac{\mathrm{d}a(t)}{\mathrm{d}t}
\text{,}
$$

and similarly for $c^*(t)$. Then

$$
x'(t) =
\frac{\mathrm{d}}{\mathrm{d}t}\left(\frac{b(t)}{a(t)}\right)
=
\frac{b^*(t)}{a(t)^2}
\text{.}
$$

$$
b^*(t) =
(w-v)
\begin{bmatrix}
    a_0b_1-a_1b_0 &
    2(a_0b_2-a_2b_0) &
    a_1b_2-a_2b_1
\end{bmatrix}
  \begin{bmatrix}
  (w-t)^2 \\
  (w-t)(t-v) \\
  (t-v)^2
  \end{bmatrix}
  \text{.}
$$

The expression in power form is very similar, though note that the
coefficients are different.

$$
b^*(t) =
(w-v)\,
\begin{bmatrix}
    a_0b_1-a_1b_0 &
    2(a_0b_2-a_2b_0) &
    a_1b_2-a_2b_1
\end{bmatrix}
  \begin{bmatrix}
  1 \\
  t \\
  t^2
  \end{bmatrix}
  \text{.}
$$

Incorporate $\sigma$.

$$
b^*(t) =
(w-v)\sigma \,
\begin{bmatrix}
    a_0b_1-a_1b_0 &
    2(a_0b_2-a_2b_0) &
    a_1b_2-a_2b_1
\end{bmatrix}
  \begin{bmatrix}
    1 & 0 & 0 \\
    0 & \sigma & 0 \\
    0 & 0 &  \sigma^2
  \end{bmatrix}
  \begin{bmatrix}
  (w-t)^2 \\
  (w-t)(t-v) \\
  (t-v)^2
  \end{bmatrix}
  \text{.}
$$

## Cubic

For CLCs, we take a different approach, in three steps. First calculate
the derivative ignoring the bilinear transformation. Second substitute
$h(t; \sigma)$ for $t$, which amounts to applying $\sigma$ much as for
the coordinate calculation. Third use chain rule, multiplying the
results by $h'(t; \sigma)$. Let the derivative be split into
$b^*(t)/a^*(t){}^2$.

$$
\begin{aligned}
\frac{\mathrm{d}}{\mathrm{d}t}\left(\frac{b(t)}{a(t)}\right)
&=
\frac{b^*(t)}{a^*(t)^2}
\\
&=
\left(\frac{  (w-t)^2(b_1-3b_0) + 2\sigma(t-v)(w-t)(b_2-b_1) + \sigma^2(t-v)^2(3b_3-b_2) }{ (w-v) ((w -t) +  \sigma (t-v))^2 } \right)
\\&\qquad\qquad
\left( \frac{ (w - v)^2 \sigma}{((w -t) +  \sigma (t-v))^2   } \right)
\end{aligned}
$$

$$
a^*(t) = ((w -t) +  \sigma (t-v))^2
$$

$$
b^*(t)
=
\sigma(w-v)\,
  \begin{bmatrix}
  b_1-3b_0 &
  2(b_2-b_1) &
  3b_3-b_2
  \end{bmatrix}
  \begin{bmatrix}
    1 & 0 & 0 \\
    0 & \sigma & 0 \\
    0 & 0 &  \sigma^2
  \end{bmatrix}
  \begin{bmatrix}
  (w-t)^2 \\
  (w-t)(t-v) \\
  (t-v)^2
  \end{bmatrix}
$$

## Velocities and end-point derivatives

The velocity of the curve is only slightly simplified by our framework.

$$
\zeta(t)
=
\frac{\sqrt{b^*(t)^2 + c^*(t)^2}}{a^*(t)^2}
\text{.}
$$

### Cubic end-points

$$
\begin{aligned}
b^*(v) &=
(w-v)^3\sigma(b_1-3b_0) \\
b^*(w) &=
(w-v)^3\sigma^3(3b_3-b_2) \\
a^*(v) &=
(w-v)^2 \\
a^*(w) &=
(w-v)^2\sigma^2
\end{aligned}
$$

### RQ end-points

Weighted.

$$
\begin{aligned}
b^*(v) &=
(w-v)^3\sigma(a_0b_1 - a_1b_0) \\
b^*(w) &=
(w-v)^3\sigma^3(a_1b_2 - a_2b_1) \\
a^*(v) &=
(w-v)^2 a_0 \\
a^*(w) &=
(w-v)^2\sigma^2 a_2
\end{aligned}
$$

## RQC cuts

### Bilinear cuts

Suppose we cut a curve with the range $[v, w]$ so that instead it begins
at $p$ and ends at $q$. The bilinear transformation of the original
curve modifies the rate of traversal. There will be a matching
transformation of the cut curve, and let $\tilde{\sigma}$ be its
parameter. Recall that the product of rate changes at the beginning and
end is constant, and that it is the ratio that conveniently
characterizes the bilinear. This ratio is

$$
\frac{h'(p; \sigma)}{h'(q; \sigma)}
=
\left(\frac%
{(w-q) + \sigma(q-v)}%
{(w-p) + \sigma(p-v)}%
\right)^2 \text{,}
$$

which is simply $\sigma^2$ when $p=v$ and $q=w$. Thus, if we cut a CLC
into range $[p, q]$ the new sigma is $$
\tilde{\sigma}=
\frac%
{(w-q) + \sigma(q-v)}%
{(w-p) + \sigma(p-v)}%
\text{.}
$$

In taking the square root we are potentially hiding sign issues. The
transformation $h(t; \sigma)$ is necessarily continuous in $[v, w]$, but
will have an asymptote iff $\sigma \neq 1$. That is, the denominator of
$h(t; \sigma)$ will be zero for a value of $t$. Let this value be
$\psi(v, w, \sigma)$. Let us express it three ways:

$$
\psi(v, w, \sigma) = \frac{w-\sigma v}{1-\sigma}
= v - \frac{w-v}{\sigma - 1} = w + \frac{\sigma(w-v)}{1-\sigma}.
$$

The first is convenient for calculation. The second shows that the
asymptote occurs at $t < v$ when $\sigma>1$, and the third that it
occurs at $t>w$ when $\sigma<1$. More specifically, addressing the
present concerns, the condition for a valid range cut is

$$
\begin{alignedat}{2}
&p > v - \frac{w-v}{\sigma - 1}       & \qquad\qquad\text{when}\quad &\sigma>1 \text{,}\\
&q < w + \frac{\sigma(w-v)}{1-\sigma} & \text{when}\quad &\sigma<1 \text{.}
\end{alignedat}
$$

This can be implemented by checking that both

$$
\begin{aligned}
&\sigma(p - \psi(v, w, \sigma)) > 0 \text{,}
\\
&\sigma(q - \psi(v, w, \sigma)) > 0 \text{.}
\end{aligned}
$$

### Making cuts

In one sense, cutting a curve is rather trivial. Really we keep the same
underlying path calculations, and “simply” change the range of $t$.
However, the range is inherent to the ways in which we express and
analyse curves. Furthermore, for CLCs we handle the bilinear
transformation as a special piece. Cutting a RQC is conceptually
straightforward. First we convert weighted to power, change the range
for $t$, then convert back to weighted form. This amounts to modifying
the curve coefficients by $\mathbf{R}_2(w,v)\mathbf{S}_2(p,q)$.

$$
\begin{split}
&\mathbf{R}_2(w,v)\,\mathbf{S}_2(p,q)
=\\
&\qquad
\frac{1}{(q-p)^2}
  \begin{bmatrix}
(w-p)^2 & 2(w-p)(w-q) & (w-q)^2 \\
(p-v)(w-p) & (q-v)(w-p) + (p-v)(w-q) & (q-v)(w-q) \\
(p-v)^2 & 2(p-v)(q-v) & (q-v)^2
\end{bmatrix}
\text{.}
\end{split}
$$

This gets complicated fast. So let us go back to the essence, expressed
earlier using matrices for the weighted form. After cutting, let us
formulate the curve, with range $[p,q]$, as follows.

$$
  \begin{bmatrix}
b(t) \\
c(t) \\
a(t)
  \end{bmatrix} =
  \left\{
  \begin{bmatrix}
  b_0 & b_1 & b_2 \\
  c_0 & c_1 & c_2 \\
  a_0 & a_1 & a_2
  \end{bmatrix}
  \,
  \mathbf{M}_2(v, w, p, q, \sigma)
  \right\}
  \begin{bmatrix}
    1 & 0 & 0 \\
    0 & \tilde{\sigma}& 0 \\
    0 & 0 &  \tilde{\sigma}^2
  \end{bmatrix}
  \begin{bmatrix}
  (q-t)^2 \\
  (q-t)(t-p) \\
  (t-p)^2
  \end{bmatrix}
  \text{.}
$$

The part in braces formulates the cut as a modification of the
coefficients. The form of $\mathbf{M}_2(v, w, p, q, \sigma)$ is

$$
  \mathbf{M}_2(v, w, p, q, \sigma)
  =
  \phi \,
  \begin{bmatrix}
    1 & 0 & 0 \\
    0 & \sigma & 0 \\
    0 & 0 &  \sigma^2
  \end{bmatrix}
  \mathbf{R}_2(w,v)\,\mathbf{S}_2(p,q)
  \begin{bmatrix}
    1 & 0 & 0 \\
    0 & \tilde{\sigma}^{-1} & 0 \\
    0 & 0 &  \tilde{\sigma}^{-2}
  \end{bmatrix}
  \text{,}
$$

where $\phi$ is a scaling factor with a specific purpose, as we will see
subsequently. Some expressions that follow can be long and hard to read.
We use the following to clarify.

$$
\begin{alignedat}{2}
\alpha &= w-p &\qquad\qquad \beta  &= \sigma(p-v) \\
\gamma &= w-q &             \delta &= \sigma(q-v)
\text{.}
\end{alignedat}
$$

Then

$$
\begin{aligned}
  &\mathbf{M}_2(v, w, p, q, \sigma)
  \\&\qquad\quad
  =
  \frac{\phi(\alpha+\beta)^2}{(q-p)^2}
   \,
  \begin{bmatrix}
\alpha^2 & 2\alpha\gamma & \gamma^2 \\
\alpha\beta  & \alpha\delta + \beta\gamma & \gamma\delta \\
\beta^2 & 2\beta\delta & \delta^2
\end{bmatrix}
  \begin{bmatrix}
    (\alpha+\beta)^{-2} & 0 & 0 \\
    0 & (\alpha+\beta)^{-1}(\gamma+\delta)^{-1} & 0 \\
    0 & 0 &  (\gamma+\delta)^{-2}
  \end{bmatrix}
  \text{,}
\end{aligned}
$$

Further let $\phi = \tfrac{(q-p)^2}{(\alpha+\beta)^2}$ and

$$
\begin{alignedat}{2}
\hat{\alpha}&= \frac{\alpha}{\alpha+\beta} &\qquad\qquad \hat{\beta}&= \frac{\beta}{\alpha+\beta} \\
\hat{\gamma}&= \frac{\gamma}{\gamma+\delta} &             \hat{\delta}&= \frac{\delta}{\gamma+\delta}
\text{.}
\end{alignedat}
$$

Hence

$$
  \mathbf{M}_2(v, w, p, q, \sigma)
  =
  \begin{bmatrix}
\hat{\alpha}^2 & 2\hat{\alpha}\hat{\gamma}& \hat{\gamma}^2 \\
\hat{\alpha}\hat{\beta}& \hat{\alpha}\hat{\delta}+ \hat{\beta}\hat{\gamma}& \hat{\gamma}\hat{\delta}\\
\hat{\beta}^2 & 2\hat{\beta}\hat{\delta}& \hat{\delta}^2
\end{bmatrix}
\text{.}
$$

## CLC cuts

For cubilinear cuts, the transformation matrix is

$$
  \mathbf{M}_3(v, w, p, q, \sigma)
  =
  \begin{bmatrix}
\hat{\alpha}^3 & 3\hat{\alpha}^2\hat{\gamma}& 3\hat{\alpha}\hat{\gamma}^2 & \hat{\gamma}^3 \\
\hat{\alpha}^2\hat{\beta}& 2\hat{\alpha}\hat{\beta}\hat{\gamma}+ \hat{\alpha}^2\hat{\delta}& 2\hat{\alpha}\hat{\gamma}\hat{\delta}+ \hat{\beta}\hat{\gamma}^2 & \hat{\gamma}^2\hat{\delta}\\
\hat{\alpha}\hat{\beta}^2 & 2\hat{\alpha}\hat{\beta}\hat{\delta}+ \hat{\beta}^2\hat{\gamma}& 2\hat{\beta}\hat{\gamma}\hat{\delta}+ \hat{\alpha}\hat{\delta}^2 & \hat{\gamma}\hat{\delta}^2 \\
\hat{\beta}^3 & 3\hat{\beta}^2\hat{\delta}& 3\hat{\beta}\hat{\delta}^2 & \hat{\delta}^3
\end{bmatrix}
\text{.}
$$

The choice of $\phi$ in the formulation of
$\mathbf{M}_2(v, w, p, q, \sigma)$ and
$\mathbf{M}_3(v, w, p, q, \sigma)$ not only simplified their forms, but
importantly it adjusted their eigenvalues. Specifically

$$
  \begin{bmatrix}
1 & 3 & 3 & 1
\end{bmatrix}
\,
  \mathbf{M}_3(v, w, p, q, \sigma)
  =
  \begin{bmatrix}
1 & 3 & 3 & 1
\end{bmatrix}
\text{.}
$$

For CLC cuts this means that we can transform the coefficients $b_0$
through $c_3$ using the preceding method. We do not need a complicated
treatment of $\sigma$, but instead replace it with $\tilde{\sigma}$.
This does what is needed to modify the denominator in the coordinate
calculation.

## Range reassignment

If we wish to change the range from $[v,w]$ to $[p,q]$ without changing
the curve, we perform a substitution in weighted form. There is no need
to adjust $\sigma$.

## Splicing

The concept, not detailed examples

<!-- ================================================================== -->

# Curve specification

Key idea is that line can only intersect normal ellipse at 2 points. No
exception for extreme case of parabola, and hyperbola line. is then
asymptote. Only collapsed curves, that is squished to eliminate one
dimension can have more. Then all end points, common points, control
points and directions that we use are co-linear.

Consequently we have not found a universal and convenient method for
specification.

1.  Four-point form does not require intersection, so can handle
    semi-ellipses. It cannot handle co-linear.
2.  Three-point forms can handle co-linear cases, but not semi-ellipses.

There is a universal form. This is a variation upon the three-point
form. The location of the intersection is specified in terms of
coordinates parallel and perpendicular to the line between end points.
The perpendicular displacement is expressed as a ratio, of which the
sign is important.

## Canonical quarter-ellipse and variants

Suppose that we let $v=0$ and $w=1$, in which case the power and
weighted forms are the same. Further let

$$
\begin{aligned}
  \begin{bmatrix}
  b(t) \\ c(t) \\ a(t)
\end{bmatrix}
&=
  \begin{bmatrix}
1 & 0 & -1 \\
0 & 2 & 0 \\
1 & 0 & 1
\end{bmatrix}
  \begin{bmatrix}
  1 \\
  t \\
  t^2
  \end{bmatrix}
  \\
  &=
  \begin{bmatrix}
1 & 2 & 0 \\
0 & 2 & 2 \\
1 & 2 & 2
\end{bmatrix}
  \begin{bmatrix}
  (w-t)^2 \\
  (w-t)(t-v) \\
  (t-v)^2
  \end{bmatrix}
\end{aligned}
$$

$$
\begin{aligned}
  \begin{bmatrix}
  b^*(t) \\ c^*(t)
\end{bmatrix}
  &=
  \begin{bmatrix}
0 & -4 & -4 \\
2 & 4 & 0
\end{bmatrix}
  \begin{bmatrix}
  (w-t)^2 \\
  (w-t)(t-v) \\
  (t-v)^2
  \end{bmatrix}
  \\
&=
  \begin{bmatrix}
0 & -4 & 0 \\
2 & 0 & -2
\end{bmatrix}
  \begin{bmatrix}
  1 \\
  t \\
  t^2
  \end{bmatrix}
\end{aligned}
$$

Then our equations match the “tan half-angle relations”, and so
$x = \cos(\theta)$, $y = \sin(\theta)$ for $t = \tan(\theta/2)$. For
$t\in (0,t)$ the curve is a standard quarter-circle, beginning at
$(1,0)$ and ending at $(0,1)$. Its derivatives are

$$
\begin{alignedat}{2}
x'(0) &= 0
&\qquad\qquad
y'(0) &= 2
\\
x'(1) &= -1
&
y'(1) &= 0
\\
x'(t) &= \frac{-4t}{(1+t^2)^2}
&
y'(t) &= \frac{2(1-t)}{(1+t^2)^2}
\text{.}
\end{alignedat}
$$

The velocity is

$$
\zeta(t) = \frac{1}{1+t^2} = \frac{1}{a(t)}
\text{.}
$$

Interestingly, the weighted expression of the velocity in terms of
$a(t)$ works for $\sigma\neq 1$. The proof requires rather fussy
manipulation.

(Aside: In general, if $b^*(t)^2 + c^*(t)^2 \propto a^*(t)^2$, this will
be true with the application of $\sigma$. Start with the weighted form,
and formulate as the outer product of coefficient vectors with
themselves. The sums of (anti) diagonal-direction sweeps must match.
These are invariant on multiplication of the coefficient vectors by the
powers of $\sigma$.)

## RGC four-point

Requirement that the end points do not coincide. No need to handle
bilinear within specification, since any sigma is separately applied.

The differences between the end points and the control points are the
derivatives scaled by $(w-v)$. We will write out a characterization of
the $x$ components, and the $y$ components are found by using $c$
coefficients instead of $b$ coefficients. Let the end points be at $x_0$
and $x_3$, and let the control points be at $x_1$ and $x_2$.

$$
\begin{alignedat}{2}
x_0 &= \frac{b_0}{a_0} &\qquad\qquad p_x &= x_1-x_0 = \frac{a_0b_1-a_1b_0}{a_0^2}
\\
x_3 &= \frac{b_2}{a_2} &  q_x &= x_3-x_2 = \frac{a_1b_2-a_2b_1}{a_2^2}
\text{.}
\end{alignedat}
$$

First, we find the $a$ coefficients using.

$$
  \begin{bmatrix}
p_x & x_0-x_3 & q_x \\
p_y & y_0-y_3 & q_y
\end{bmatrix}
  \begin{bmatrix}
  a_0 \\ a_1 \\ a_2
\end{bmatrix}
=
\begin{bmatrix}
0 \\ 0
\end{bmatrix}
\text{.}
$$

We can pre-multiply the 2-by-3 matrix here by

$$
\begin{bmatrix}
x_0-x_3 & y_0-y_3 \\
-(y_0-y_3) & x_0-x_3
\end{bmatrix}
\text{,}
$$

which is in essence an unscale Givens rotation, and is guaranteed to be
non-singular if the end points do not coincide. Multiplying by this
zeros out an entry, and so we can assign $a_0$ and $a_2$, with arbitrary
scaling.

The remaining coefficients can be obtained fromt he above, first $b_0$
and $b_3$, and then

$$
b_1 = a_0 p_x + a_1x_0 \text{.}
$$

The $c$ coefficients can be obtained in like manner

## Three-point RGC

Take canonical quarter-circle, select $\tau$. Length of end points to
common point is $\tau$. Derivatives is $1/(1+t^2)$, and size of range is
$\tau$. Therefore starting line length is $\tau$ and end line length is
$\tau/(1+\tau^2)$. Equivalent 4-point specification is to have first
control point at common point, and second dividing its line into the
ratio $1:\tau^2$.

## Three-point RGC calculation

Let the *common* third point be at $(x_c, y_c)$, and let this be the
intersection of the lines from the end points through their
corresponding control points. Then let $\lambda_1$ and $\lambda_2$ be
such that

$$
% \begin{alignedat}{2}
x_1-x_0 = \lambda_1(x_c-x_0) = p_x\qquad\qquad
x_3-x_2 = \lambda_2(x_3-x_c) = q_x
\text{,}
% \end{alignedat}
$$

with similar for $y$ components. Then we need to solve

$$
  \begin{bmatrix}
\lambda_1(x_c-x_0) & x_0-x_3 & \lambda_2(x_3-x_c) \\
\lambda_1(y_c-y_0) & y_0-y_3 & \lambda_2(y_3-y_c)
\end{bmatrix}
  \begin{bmatrix}
  a_0 \\ a_1 \\ a_2
\end{bmatrix}
=
\begin{bmatrix}
0 \\ 0
\end{bmatrix}
\text{.}
$$

Hence

$$
%\begin{alignedat}{3}
a_0 = \lambda_2\qquad\qquad
a_1 = \lambda_1\lambda_2\qquad\qquad
a_2 = \lambda_1
\text{.}
%\end{alignedat}
$$

The $b$ and $c$ coefficients can be found in the same way as for the
four-point task.

## Cutting corners

## Cubilinear

## Straight lines

<!-- ================================================================== -->

# Elaborated topics

## Matching velocities

<!-- ================================================================== -->

# Other (maybe) handy stuff

## Bilinear transformation

Matrix (Möbius) form.

$$
\begin{aligned}
h(t; \sigma) &=
\left(
  \begin{bmatrix}
  \sigma w-v & (1-\sigma)vw \\
  \sigma-1 & w-\sigma v
  \end{bmatrix}
  \right)
  \begin{bmatrix}
  t \\
  1
  \end{bmatrix}
  \\
  &=
\left(
\sigma \,
  \begin{bmatrix}
  w & -vw \\
  1 & -v
  \end{bmatrix}
 +
  \begin{bmatrix}
  -v & +vw \\
  -1 & w
  \end{bmatrix}
  \right)
  \begin{bmatrix}
  t \\
  1
  \end{bmatrix}
\end{aligned}
$$

Our preferred form, which matches the power form of curves, exchanges
rows and columns.

$$
  \begin{bmatrix}
  \gamma \\
\gamma h(t; \sigma)
  \end{bmatrix}
   =
   \frac{1}{(w-v)}
\begin{bmatrix}
1 & 1\\
v & w
\end{bmatrix}
  \begin{bmatrix}
  \sigma & 0 \\
  0 & 1
  \end{bmatrix}
\begin{bmatrix}
w & -1\\
-v & 1
\end{bmatrix}
  \begin{bmatrix}
  1 \\
  t
  \end{bmatrix}
$$

Range $[0, r]$.

$$
\begin{aligned}
h(t; \sigma) &= \frac{%
\sigma rt
}{%
(\sigma-1)t + r
}
\\
h'(t; \sigma) &= \frac{%
r^2 \sigma
}{%
((\sigma-1)t + r)^2
}
\end{aligned}
$$

Range $[-r, r]$.

$$
\begin{aligned}
h(t; \sigma) &= \frac{%
((\sigma  +1)t + (\sigma-1) r)r
}{%
(\sigma-1)t + (\sigma +1)r
}
\\
h'(t; \sigma) &= \frac{%
4r^2 \sigma
}{%
((\sigma-1)t + (\sigma +1)r)^2
}
\end{aligned}
$$

## Traditional power forms and derivatives

$$
g'(t) = 3(1-t)^2(p_1-p_0) + 6t(1-t)(p_2-p_1) + 3t^2(p_3-p_2) \label{Eq02}
$$

And

$$
f'(t) = \frac{%
2w_Aw_B(1-t)^2(p_m-p_0) + 2w_Aw_Ct(1-t)(p_3-p_0) + 2w_Bw_Ct^2(p_3-p_m)
}{%
(w_A(1-t)^2 + 2w_Bt(1-t) + w_Ct^2)^2
}\label{Eq04}
$$

Likely defunct cubic power form

$$
  \begin{bmatrix}
x(t) \\
y(t)
  \end{bmatrix} =
  \begin{bmatrix}
  b_0 & b_1 & b_2 & b_3 \\
  c_0 & c_1 & c_2 & c_3
  \end{bmatrix}
  \begin{bmatrix}
  1 \\
  t \\
  t^2 \\
  t^3
  \end{bmatrix}
$$

## Degree-1 matrix conversions

$$
  \begin{bmatrix}
  w-t \\
  t-v
  \end{bmatrix}
  =
\begin{bmatrix}
w & -1\\
-v & 1
\end{bmatrix}
  \begin{bmatrix}
  1 \\
  t
  \end{bmatrix}
$$

$$
  \begin{bmatrix}
  1 \\
  t
  \end{bmatrix}
  =
  \frac{1}{(w-v)}
\begin{bmatrix}
1 & 1\\
v & w
\end{bmatrix}
  \begin{bmatrix}
  w-t \\
  t-v
  \end{bmatrix}
$$

## Range cut

The basic method for cutting a curve is to convert into power form. In
power form the range can, aside from bilinear transformation, be simply
substituted. However, the bilinear factor $\sigma$ needs to be
considered. If we cut the range $[v,w]$ into a new range $[p,q]$, we
need to match velocities. The original curve has a speed up of $\sigma$
at the start and like slowdown at the end.

(cut)

Let $\mu$ be the new bilinear transformation parameter. This needs to
satisfy $$
\mu^2 = h'(p; \sigma) \, / \, h'(q; \sigma)
$$

This is because $\mu$ must create a *relative* speed-up of
$h'(p; \sigma)$ at the beginning of the new curve and a *relative*
slowdown of $h'(q; \sigma)$ at the end of the curve. Therefore

$$
\begin{aligned}
\mu^2 &=
\frac{%
((\sigma-1)q + (w - \sigma v))^2
}{%
((\sigma-1)p + (w - \sigma v))^2
} \\
\mu &=
\frac{%
(w-q) + \sigma (q-v)
}{%
(w-p) + \sigma (p-v)
}
\end{aligned}
$$

There will only be a problem if either the numerator or the denominator
is negative. This will never be the case for a valid range cut. What
makes a range cut invalid?

It is guaranteed that $h(t; \sigma)$ is monotonically increasing between
$v$ and $w$, but it is not guaranteed outside of this range.
Specifically, $h'(t; \sigma)$ becomes infinite when
$t= \psi(v, w, \sigma)$, where

(cut)

This makes intuitive sense. For the first case the bilinear transform
slows the curve at the beginning, and increases the velocity. If $t$
goes beyond $w$, the velocity adjustment increases to infinity at
$\psi(v, w, \sigma)$.

## Derivative bits

RQ numerator

$$
b^*(t) =
(w-v)
\begin{bmatrix}
b_0 & b_1 & b_2
\end{bmatrix}
\begin{bmatrix}
0 & -(w-t)^2 & -2(t-v)(w-t) \\
(w-t)^2 & 0 & -(t-v)^2 \\
2(t-v)(w-t) & (t-v)^2 & 0
\end{bmatrix}
\begin{bmatrix}
  a_0 \\ a_1 \\ a_2
\end{bmatrix}
$$

Conversion from power to weighted.

$$
\begin{aligned}
\begin{bmatrix}
w^2 & -2w & 1\\
-vw & (v+w) & -1\\
v^2 & -2v & 1
\end{bmatrix}
&
\begin{bmatrix}
0 & -1 & -2t \\
1 & 0 & -t^2 \\
2t & t^2 & 0
\end{bmatrix}
\begin{bmatrix}
w^2 & -vw & v^2\\
-2w & (v+w) & -2v\\
1 & -1 & 1
\end{bmatrix}
\\
&\qquad=\quad
(w-v)
\begin{bmatrix}
0 & -(w-t)^2 & -2(t-v)(w-t) \\
(w-t)^2 & 0 & -(t-v)^2 \\
2(t-v)(w-t) & (t-v)^2 & 0
\end{bmatrix}
\end{aligned}
$$

<!-- ============================================================ -->
## Cutting corners

We can pull out useful relations if we begin with the quarter circle and
cut a new range $[0, \tau]$. Let the start and end points be
$\mathrm{S}$ and $\mathrm{F}$, and let the tangents at these points meet
at point $\mathrm{M}$. The lines $\mathrm{SM}$ and $\mathrm{MF}$ have
length $\tau$ and the line $\mathrm{SF}$ has length equal to twice the
sine of the half angle. That is,

$$
\bigl\lvert \mathrm{SF} \bigr\rvert
=
\frac{2\tau}{\sqrt{1+\tau^2}}
\text{.}
$$

If we divide this by 3 and square, we get the product of the lengths of
the control lines.

$$
\left(\tfrac{1}{3}\bigl\lvert \mathrm{SF}\bigr\rvert \right)^2
=
\Biggl(\frac{2}{3}\tau\Biggr)
\Biggl(\frac{2\tau}{3(1+\tau^2)}\Biggr)
\text{.}
$$

This is the “division into thirds” rule. It matches the rule that we
found before for three-point RGCs. The needed proportions are

$$
\lambda_1 = \frac{2}{3}
\qquad\qquad
\lambda_2 = \frac{2}{3(1+\tau^2)}
\text{.}
$$

Furthermore, this naturally becomes the parabolic special case when
$\tau = 0$.

## Cubilinear curve specification

$$
  \begin{aligned}
3(x_1-x_0) &= (w-v)x'(v) = b_1-3b_0
\\
3(x_3-x_2) &= (w-v)x'(w) = 3b_3-b_2
\end{aligned}
$$

$$
  \begin{bmatrix}
  b_0 & b_1 & b_2 & b_3 \\
  c_0 & c_1 & c_2 & c_3
\end{bmatrix}
=
  \begin{bmatrix}
  x_0 & x_1 & x_2 & x_3 \\
  y_0 & y_1 & y_2 & y_3
\end{bmatrix}
  \begin{bmatrix}
  1 & 0 & 0 & 0 \\
  0 & 3 & 0 & 0 \\
  0 & 0 & 3 & 0 \\
  0 & 0 & 0 & 1
\end{bmatrix}
\text{.}
$$

$$
  \begin{aligned}
  \begin{bmatrix}
  b^*_0 & b^*_1 & b^*_2 \\
  c^*_0 & c^*_1 & c^*_2
\end{bmatrix}
&=
(w-v)
  \begin{bmatrix}
  b_0 & b_1 & b_2 & b_3 \\
  c_0 & c_1 & c_2 & c_3
\end{bmatrix}
  \begin{bmatrix}
  -3 & 0 & 0 \\
  1 & -2 & 0 \\
  0 & 2 & -1 \\
  0 & 0 & 3
\end{bmatrix}
\\
&=
(w-v)
  \begin{bmatrix}
  x_0 & x_1 & x_2 & x_3 \\
  y_0 & y_1 & y_2 & y_3
\end{bmatrix}
  \begin{bmatrix}
  -3 & 0 & 0 \\
  3 & -6 & 0 \\
  0 & 6 & -3 \\
  0 & 0 & 3
\end{bmatrix}
\end{aligned}
$$

## RQC solvable split

$$
  \begin{bmatrix}
  b_0 & b_1 & b_2 \\
  c_0 & c_1 & c_2 \\
  a_0 & a_1 & a_2
\end{bmatrix}
=
  \begin{bmatrix}
  x_0 & x_c & x_3 \\
  y_0 & y_c & y_3 \\
  1 & 1 & 1
\end{bmatrix}
  \begin{bmatrix}
  \lambda_2 & 0 & 0 \\
  0 & \lambda_1 \lambda_2 & 0 \\
  0 & 0 & \lambda_1
\end{bmatrix}
\text{.}
$$

$$
  \begin{bmatrix}
  b^*_0 & b^*_1 & b^*_2 \\
  c^*_0 & c^*_1 & c^*_2
\end{bmatrix}
=
(w-v)
  \begin{bmatrix}
  b_0 & b_1 & b_2 \\
  c_0 & c_1 & c_2
\end{bmatrix}
  \begin{bmatrix}
  -a_1 & -2a_2 & 0 \\
  a_0 & 0 & -a_2 \\
  0 & 2a_0 & a_1
\end{bmatrix}
\text{.}
$$
