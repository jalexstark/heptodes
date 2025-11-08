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
title: A Compendium of Curve Mathematics in Zebraix
author:
- J. Alex Stark
date: 2025
...

<!-- mdformat on -->

# Task

## Traditional power forms

$$
g(t) = (1-t)^3p_0 + 3t(1-t)^2p_1 + 3t^2(1-t)p_2 + t^3p_3 \text{.} \label{Eq01}
$$

$$
g'(t) = 3(1-t)^2(p_1-p_0) + 6t(1-t)(p_2-p_1) + 3t^2(p_3-p_2) \label{Eq02}
$$

For RQCs, we use a three-point expression along with three weights ($w_A$,
$w_B$, $w_C$). The path and its derivative are


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


## Power and weighted forms

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

Weighted form, $t\in[v,w]$.

$$
  \begin{bmatrix}
x(t) \\
y(t)
  \end{bmatrix} =
  \frac{1}{(w-v)^3}
  \begin{bmatrix}
  b_0 & b_1 & b_2 & b_3 \\
  c_0 & c_1 & c_2 & c_3
  \end{bmatrix}
  \begin{bmatrix}
  (w-t)^3 \\
  (w-t)^2(t-v) \\
  (w-t)(t-v)^2 \\
  (t-v)^3
  \end{bmatrix}
$$

### Homogeneous form, cubic

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

### Homogeneous form, rat-quad

Weighted forms.  Power form similar, but only $\sigma=1$.

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
$$




# Bilinear

General for range $[u, w]$.

$$
h(t; \sigma) = \frac{%
(\sigma w -v)t - (\sigma-1) vw
}{%
(\sigma-1)t + (w - \sigma v)
}
$$

Constraints.
$$
\begin{aligned}
h(w; \sigma) &= w \\
h(v; \sigma) &= v \\
h(h(t; \sigma_1); \sigma_2) &= h(t; \sigma_1 \sigma_2) \\
h(h(t; \sigma); \sigma^{-1}) &= t
\end{aligned}
$$

Derivative.
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



# Conversions

## Power


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
  (w-t)^2 \\
  (w-t)(t-v) \\
  (t-v)^2
  \end{bmatrix}
  =
\begin{bmatrix}
w^2 & -2w & 1\\
-vw & v+w & -1\\
v^2 & -2v & 1
\end{bmatrix}
  \begin{bmatrix}
  1 \\
  t \\
  t^2
  \end{bmatrix}
$$


$$
  \begin{bmatrix}
  (w-t)^3 \\
  (w-t)^2(t-v) \\
  (w-t)(t-v)^2 \\
  (t-v)^3
  \end{bmatrix}
  =
\begin{bmatrix}
w^3 &  -3w^2 & 3w & -1\\
-vw^2 & w(2v+w) & -(v+2w) & 1\\
v^2w & -v(v+2w) & (2v+w) & -1\\
-v^3 &  3v^2 & -3v & 1
\end{bmatrix}
  \begin{bmatrix}
  1 \\
  t \\
  t^2 \\
  t^3
  \end{bmatrix}
$$

## Weighted

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

$$
  \begin{bmatrix}
  1 \\
  t \\
  t^2
  \end{bmatrix}
  =
  \frac{1}{(w-v)^2}
\begin{bmatrix}
1 & 2 & 1 \\
v & v+w & w \\
v^2 & 2vw & w^2
\end{bmatrix}
  \begin{bmatrix}
  (w-t)^2 \\
  (w-t)(t-v) \\
  (t-v)^2
  \end{bmatrix}
$$


$$
  \begin{bmatrix}
  1 \\
  t \\
  t^2 \\
  t^3
  \end{bmatrix}
  =
  \frac{1}{(w-v)^3}
\begin{bmatrix}
1 &  3 & 3 & 1\\
v & 2v+w & v+2w & w\\
v^2 & v(v+2w) & w(2v+w) & w^2\\
v^3 &  3v^2w & 3vw^2 & w^3
\end{bmatrix}
  \begin{bmatrix}
  (w-t)^3 \\
  (w-t)^2(t-v) \\
  (w-t)(t-v)^2 \\
  (t-v)^3
  \end{bmatrix}
$$



# Conversion

RatQuad as example.

## Convert weighted to power

Weighted path on left, multiply on right by this.

$$
\begin{bmatrix}
w^2 & -2w & 1\\
-vw & v+w & -1\\
v^2 & -2v & 1
\end{bmatrix}
$$

## Convert power to weighted

Power path on left, multiply on right by this.  Scaled inverse of
weighted-to-power.  Scaling factor is $(v-w)^2$.

$$
  \frac{1}{(w-v)^2}
\begin{bmatrix}
1 & 2 & 1 \\
v & v+w & w \\
v^2 & 2vw & w^2
\end{bmatrix}
$$

# Range manipulation

## Range cut

The basic method for cutting a curve is to convert into power form.
In power form the range can, aside from bilinear transformation, be
simply substituted.  However, the bilinear factor $\sigma$ needs to be
considered.  If we cut the range $[v,w]$ into a new range $[p,q]$, we
need to match velocities.  The original curve has a speed up of
$\sigma$ at the start and like slowdown at the end.

Let $\mu$ be the new bilinear transformation parameter.  This needs to
satisfy
$$
\mu^2 = h'(p; \sigma) \, / \, h'(q; \sigma)
$$

This is because $\mu$ must create a *relative* speed-up of $h'(p;
\sigma)$ at the beginning of the new curve and a *relative* slowdown
of $h'(q; \sigma)$ at the end of the curve.  Therefore

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

There will be a problem if either the numerator or the denominator is
negative.  This will always be the case for a valid range cut.  What
makes a range cut invalid?

It is guaranteed that $h(t; \sigma)$ is monotonically increasing
between $v$ and $w$, but it is not guaranteed outside of this range.
Specifically, $h'(t; \sigma)$ becomes infinite when $t= \phi(v, w,
\sigma)$, where

$$
\phi(v, w, \sigma) = \frac{w-\sigma v}{1-\sigma}
$$

Recall that both $w-v$ and $\sigma$ must be positive.  When $\sigma <
1$ all terms are positive in

$$
t-w = \frac{\sigma(w-v)}{1-\sigma}
$$

On the other hand, when $\sigma>1$ all terms are positive in

$$
v-t = \frac{w-v}{\sigma - 1}
$$

This makes intuitive sense.  For the first case the bilinear transform
slows the curve at the beginning, and increases the velocity.  If $t$
goes beyond $w$, the velocity adjustment increases to infinity at
$\phi(v, w, \sigma)$.

Therefore, in summary, one of two bounds must be satisfied on $p$ and
$q$ (in addition to $p < q$).  These are that

$$
\begin{aligned}
    p &> \phi(v, w, \sigma) \qquad \sigma > 1 \\
    q &< \phi(v, w, \sigma) \qquad \sigma < 1
\end{aligned}
$$

Finally, observe that if $\sigma = 1$ then $\mu=1$.

## Range reassignment

If we wish to change the range from $[v,w]$ to $[p,q]$ without
changing the curve, we perform a substitution in weighted form.  There
is no need to adjust $\sigma$.

# Derivatives

## Cubic

If needing more than end-points, the method is first to bilinear
transform $t$, calculate the basic derivative. Then use chain rule,
multiplying by $h'(t; \sigma)$.

$$
\begin{aligned}
b'(t) &= (w-t)^2(b_1-3b_0) + 2(t-v)(w-t)(b_2-b_1) + (t-v)^2(3b_3-b_2)
\\
a'(t) &= (w-v)^2
\end{aligned}
$$

## RQ

RQ numerator

$$
\begin{aligned}
b'(t) &=
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
\\ &=
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
\end{aligned}
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

So power form is very similar to weighted in this aspect.

$$
b'(t) =
(w-v)
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
$$


Denominator is square of the non-differentiated denominator.  In
essence this is the same for power and weighted.

$$
a'(t) =
\left(
\begin{bmatrix}
a_0 & a_1 & a_2
\end{bmatrix}
  \begin{bmatrix}
  1 \\
  t \\
  t^2
  \end{bmatrix}
\right)
^2
$$

## Cubic end-points

$$
\begin{aligned}
b'(v) &=
(w-v)^3\sigma(b_1-3b_0) \\
b'(w) &=
(w-v)^3\sigma^2(3b_3-b_2) \\
a'(v) &=
(w-v)^2 \\
a'(w) &=
(w-v)^2\sigma^3
\end{aligned}
$$

## RQ end-points

Weighted.

$$
\begin{aligned}
b'(v) &=
(w-v)^2\sigma(a_0b_1 - a_1b_0) \\
b'(w) &=
(w-v)^2\sigma^3(a_1b_2 - a_2b_1) \\
a'(v) &=
(w-v)^2 a_0{}^2 \\
a'(w) &=
(w-v)^2\sigma^4 a_2{}^2
\end{aligned}
$$
