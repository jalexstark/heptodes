% Rational Quadratic Splines and Bézier Splines
% J. Alex Stark
% 2003--2022


----------------------------------------

Heptodes documents and other content in `doc` directories are licensed under the
[Creative Commons Attribution 4.0 License] (CC BY 4.0 license).

Source code licensed and code samples are licensed under the [Apache
2.0 License].

The CC BY 4.0 license requires attribution.  When samples, examples, figures,
tables, or other excerpts, are used in a tutorial, or a subdivision thereof, it
is sufficient to provide the complete source and license information once.  This
must be close to the beginning, such as in an early acknowledgments slide.  If
this is done, only short notes are required to be placed with each usage, such
as in figure captions.

[Creative Commons Attribution 4.0 License]: https://creativecommons.org/licenses/by/4.0/legalcode
[Apache 2.0 License]: https://www.apache.org/licenses/LICENSE-2.0

----------------------------------------


# Figures

![XXXXXXXXXXXXXX\label{figA}.](figs/RatQuad-A.svg)

![XXXXXXXXXXXXXX\label{figB}.](figs/RatQuad-B.svg)

![XXXXXXXXXXXXXX\label{figD}.](figs/RatQuad-D.svg)

![XXXXXXXXXXXXXX\label{figE}.](figs/RatQuad-E.svg)

![XXXXXXXXXXXXXX\label{figF}.](figs/RatQuad-F.svg)

# Equations

Equation \ref{Eq01}:Eq01.

$$
g(t) = (1-t)^3p_0 + 3t(1-t)^2p_1 + 3t^2(1-t)p_2 + t^3p_3 \label{Eq01}
$$

Equation \ref{Eq02}:Eq02.

$$
g'(t) = 3(1-t)^2(p_1-p_0) + 6t(1-t)(p_2-p_1) + 3t^2(p_3-p_2) \label{Eq02}
$$

Equation \ref{Eq03}:Eq03.

$$
f(t) = \frac{%
w_A(1-t)^2p_0 + 2w_Bt(1-t)p_m + w_Ct^2p_3
}{%
w_A(1-t)^2 + 2w_Bt(1-t) + w_Ct^2
} \label{Eq03}
$$

Equation \ref{Eq04}:Eq04.

$$
f'(t) = \frac{%
2w_Aw_B(1-t)^2(p_m-p_0) + 2w_Aw_Ct(1-t)(p_3-p_0) + 2w_Bw_Ct^2(p_3-p_m)
}{%
(w_A(1-t)^2 + 2w_Bt(1-t) + w_Ct^2)^2
}\label{Eq04}
$$

Equation \ref{Eq05}:Eq05.

$$
g'(0) = 3(p_1-p_0) \label{Eq05}
$$

Equation \ref{Eq06}:Eq06.

$$
g'(1) = 3(p_3-p_2) \label{Eq06}
$$

Equation \ref{Eq07}:Eq07.

$$
f'(0) = \frac{2w_B}{w_A}(p_m-p_0) \label{Eq07}
$$

Equation \ref{Eq08}:Eq08.

$$
f'(1) = \frac{2w_B}{w_C}(p_3-p_m) \label{Eq08}
$$

Equation \ref{Eq09}:Eq09.

$$
p_1-p_0 = \lambda(p_m-p_0) \label{Eq09}
$$

Equation \ref{Eq10}:Eq10.

$$
p_2-p_3 = \mu(p_m-p_3) \label{Eq10}
$$

Equation \ref{Eq11}:Eq11.

$$
g'(0) = f'(0) \label{Eq11}
$$

Equation \ref{Eq12}:Eq12.

$$
g'(1) = f'(1) \label{Eq12}
$$

Equation \ref{Eq13}:Eq13.

$$
\lambda = \frac{2w_B}{3w_A} \label{Eq13}
$$

Equation \ref{Eq14}:Eq14.

$$
\mu = \frac{2w_B}{3w_C} \label{Eq14}
$$

Equation \ref{Eq15}:Eq15.

$$
g'(t) = 3\lambda(1-t)^2(p_m-p_0) + 6t(1-t)((1-\mu)p_3-(1-\lambda)p_0+(\mu-\lambda)p_m) - 3\mu t^2(p_m-p_3) \label{Eq15}
$$

Equation \ref{Eq16}:Eq16.

$$
\lambda = \mu = \frac23 \label{Eq16}
$$

Equation \ref{Eq17}:Eq17.

$$
w_A = w_B = w_C \label{Eq17}
$$

Equation \ref{Eq18}:Eq18.

$$
f(t) = g(t) = (1-t)^2p_0 + 2t(1-t)p_m + t^2p_3 \label{Eq18}
$$

Equation \ref{Eq19}:Eq19.

$$
d = p_3-p_0 \label{Eq19}
$$

Equation \ref{Eq20}:Eq20.

$$
n\cdot d = 0 \label{Eq20}
$$

Equation \ref{Eq21}:Eq21.

$$
a = p_1-p_0 = \lambda(p_m-p_0) \label{Eq21}
$$

Equation \ref{Eq22}:Eq22.

$$
b = p_2-p_3 = \lambda(p_m-p_3) \label{Eq22}
$$

Equation \ref{Eq23}:Eq23.

$$
d = \frac{a}{\lambda} - \frac{b}{\mu} \label{Eq23}
$$

Equation \ref{Eq24}:Eq24.

$$
\frac{a\cdot n}{\lambda} - \frac{b\cdot n}{\mu} = 0 \label{Eq24}
$$

Equation \ref{Eq25}:Eq25.

$$
\frac{a\cdot d}{\lambda} - \frac{b\cdot d}{\mu} = \left| d\right|^2 \label{Eq25}
$$

Equation \ref{Eq26}:Eq26.

$$
\lambda = \frac{\gamma\left| d\right|^2}{b\cdot n} \label{Eq26}
$$

Equation \ref{Eq27}:Eq27.

$$
\lambda = \frac{\gamma\left| d\right|^2}{a\cdot n} \label{Eq27}
$$

Equation \ref{Eq28}:Eq28.

$$
\gamma = \frac{(a\cdot d)(b\cdot n)-(b\cdot d)(a\cdot n)}{\left| d\right|^4} \label{Eq28}
$$

Equation \ref{Eq29}:Eq29.

$$
w_A = \frac{3(b\cdot n)}{2\left| d\right|^2} \label{Eq29}
$$

Equation \ref{Eq30}:Eq30.

$$
w_B = \left(\frac{3}{2}\right)^2\gamma \label{Eq30}
$$

Equation \ref{Eq31}:Eq31.

$$
w_C = \frac{3(a\cdot n)}{2\left| d\right|^2} \label{Eq31}
$$

Equation \ref{Eq32}:Eq32.

$$
w_A \neq 0 \label{Eq32}
$$

Equation \ref{Eq33}:Eq33.

$$
w_C \neq 0 \label{Eq33}
$$

Equation \ref{Eq34}:Eq34.

$$
f_Q(t) = \begin{bmatrix}
\frac{1-t^2}{1+t^2} \\
\frac{2t}{1+t^2}
\end{bmatrix} \label{Eq34}
$$

Equation \ref{Eq35}:Eq35.

$$
f_S(t) = \begin{bmatrix}
\frac{2t(1-t)}{t^2+(1-t)^2} \\
\frac{t^2-(1-t)^2}{t^2+(1-t)^2}
\end{bmatrix} \label{Eq35}
$$

Equation \ref{Eq36}:Eq36.

$$
v(t) = \frac{\sigma t}{1+(\sigma -1)t} \label{Eq36}
$$

Equation \ref{Eq37}:Eq37.

$$
f_\sigma(t) = f(v(t)) =\frac{%
w_A(1-t)^2p_0 + 2\sigma w_Bt(1-t)p_M + \sigma^2 w_Ct^2p_3
}{%
w_A(1-t)^2 + 2\sigma w_Bt(1-t) + \sigma^2 w_Ct^2
} \label{Eq37}
$$

Equation \ref{Eq38}:Eq38.

$$
f'_\sigma(0) = \left(\frac{2\sigma w_B}{w_A}\right)(p_m-p_0) \label{Eq38}
$$

Equation \ref{Eq39}:Eq39.

$$
f'_\sigma(1) = \left(\frac{2w_B}{\sigma w_C}\right)(p_3-p_m) \label{Eq39}
$$

Equation \ref{Eq40}:Eq40.

$$
f'_\sigma(0)f'_\sigma(1) = f'(0)f'(1) \label{Eq40}
$$

Equation \ref{Eq41}:Eq41.

$$
f_Q(t) = \frac{%
(1-t)^2\begin{bmatrix}1\\0\end{bmatrix} + 2t(1-t)\begin{bmatrix}1\\1\end{bmatrix} + 2t^2\begin{bmatrix}0\\1\end{bmatrix}
}{%
(1-t)^2 + 2t(1-t) + 2t^2
} \label{Eq41}
$$

Equation \ref{Eq42}:Eq42.

$$
f_S(t) = \begin{bmatrix}
\frac{1-(2t-1)^2}{1+(2t-1)^2} \\
\frac{2(2t-1)}{1+(2t-1)^2}
\end{bmatrix}  \label{Eq42}
$$

Equation \ref{Eq43}:Eq43.

$$
f_S(t) = \frac{%
(1-t)^2\begin{bmatrix}0\\-1\end{bmatrix} + 2t(1-t)\begin{bmatrix}1\\0\end{bmatrix} + t^2\begin{bmatrix}0\\1\end{bmatrix}
}{%
(1-t)^2 + (0)2t(1-t) + t^2
}  \label{Eq43}
$$

Equation \ref{Eq44}:Eq44.

$$
f_\delta(t) = \frac{%
(1-t)^2\begin{bmatrix}r\\0\end{bmatrix} + 2t(1-t)\begin{bmatrix}r\\r\delta\end{bmatrix} + (1+\delta^2)t^2\left(\frac{1}{1+\delta^2}\right)\begin{bmatrix}r(1-\delta^2)\\2r\delta\end{bmatrix}
}{%
(1-t)^2 + 2t(1-t) + (1+\delta^2)t^2
} \label{Eq44}
$$

Equation \ref{Eq45}:Eq45.

$$
f'_\delta(t) = r\delta f'_Q(\delta t)\label{Eq45}
$$

Equation \ref{Eq46}:Eq46.

$$
f'_Q(t) = 2\begin{bmatrix}
\frac{-2t}{(1+t^2)^2} \\
\frac{1-t^2}{(1+t^2)^2}
\end{bmatrix} \label{Eq46}
$$

Equation \ref{Eq47}:Eq47.

$$
f'_\delta(0) = 2r\delta\begin{bmatrix}
0\\1
\end{bmatrix} \label{Eq47}
$$

Equation \ref{Eq48}:Eq48.

$$
f'_\delta(1) = \left(\frac{2r\delta}{1+\delta^2}\right)%
\begin{bmatrix}
\frac{-2\delta}{1+\delta^2} \\
\frac{1-\delta^2}{1+\delta^2}
\end{bmatrix} \label{Eq48}
$$

Equation \ref{Eq49}:Eq49.

$$
\left| f'_\delta(1) \right| = \frac{2r\delta}{1+\delta^2} \label{Eq49}
$$

Equation \ref{Eq50}:Eq50.

$$
3\lambda\left|p_m-p_0\right|\,3\mu\left|p_m-p_3\right| =
\left|f'_\delta(0)\right|\,\left|f'_\delta(1)\right| \label{Eq50}
$$

Equation \ref{Eq51}:Eq51.

$$
9\lambda\mu(r\delta)^2 = \frac{4(r\delta)^2}{1+\delta^2} \label{Eq51}
$$

Equation \ref{Eq52}:Eq52.

$$
\lambda\mu = \frac{4}{9(1+\delta^2)} \label{Eq52}
$$

Equation \ref{Eq53}:Eq53.

$$
\lambda\mu = \frac{2}{9}(1-\cos(\Omega)) \label{Eq53}
$$

Equation \ref{Eq54}:Eq54.

$$
\left|p_1-p_0\right| = \frac13(a+b-c) \label{Eq54}
$$

Equation \ref{Eq55}:Eq55.

$$
\left|p_2-p_3\right| = \frac13(a-b+c) \label{Eq55}
$$

Equation \ref{Eq56}:Eq56.

$$
9\left|p_1-p_0\right|\,\left|p_2-p_3\right| =
2bc-(b^2+c^2-a^2) \label{Eq56}
$$

Equation \ref{Eq57}:Eq57.

$$
9\left|p_1-p_0\right|\,\left|p_2-p_3\right| =
2bc(1-\cos(\Omega)) \label{Eq57}
$$

Equation \ref{Eq58}:Eq58.

$$
9\left|p_1-p_0\right|\,\left|p_2-p_3\right| =
2\lambda\mu bc \label{Eq58}
$$

Equation \ref{Eq59}:Eq59.

$$
a\cdot n \neq 0 \label{Eq59}
$$

Equation \ref{Eq60}:Eq60.

$$
b\cdot n \neq 0 \label{Eq60}
$$

Equation \ref{Eq61}:Eq61.

$$
p_n = w_Bp_m =
\left(\frac32\right)^2\gamma\left(
p_0 + \left(\frac{b\cdot n}{\gamma \left| d\right|^2}\right)(p_1-p_0)
\right) \label{Eq61}
$$

Equation \ref{Eq62}:Eq62.

$$
p_n =
\left(\frac32\right)^2\left(
\gamma\, p_0 + \left(\frac{b\cdot n}{\left| d\right|^2}\right) a
\right) \label{Eq62}
$$

Equation \ref{Eq63}:Eq63.

$$
p_n =
\left(\frac32\right)^2\left(
\gamma\, p_3 + \left(\frac{a\cdot n}{\left| d\right|^2}\right) b
\right) \label{Eq63}
$$

Equation \ref{Eq64}:Eq64.

$$
a = \left|p_3-p_0\right| \label{Eq64}
$$

Equation \ref{Eq65}:Eq65.

$$
b = \left|p_m-p_0\right| \label{Eq65}
$$

Equation \ref{Eq66}:Eq66.

$$
c = \left|p_m-p_3\right| \label{Eq66}
$$

Equation \ref{Eq67}:Eq67.

$$
XXX \label{Eq67}
$$


Equation \ref{Eq68}:Eq68.

$$
XXX \label{Eq68}
$$

Equation \ref{Eq69}:Eq69.

$$
XXX \label{Eq69}
$$

Equation \ref{Eq70}:Eq70.

$$
XXX \label{Eq70}
$$

Equation \ref{Eq71}:Eq71.

$$
XXX \label{Eq71}
$$

Equation \ref{Eq72}:Eq72.

$$
XXX \label{Eq72}
$$

Equation \ref{Eq73}:Eq73.

$$
XXX \label{Eq73}
$$

Equation \ref{Eq74}:Eq74.

$$
XXX \label{Eq74}
$$

Equation \ref{Eq75}:Eq75.

$$
XXX \label{Eq75}
$$

Equation \ref{Eq76}:Eq76.

$$
XXX \label{Eq76}
$$

Equation \ref{Eq77}:Eq77.

$$
XXX \label{Eq77}
$$


Equation \ref{Eq78}:Eq78.

$$
XXX \label{Eq78}
$$

Equation \ref{Eq79}:Eq79.

$$
XXX \label{Eq79}
$$
