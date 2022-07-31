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


# Figures and tables

![XXXXXXXXXXXXXX\label{figA}.](figs/RatQuad-A.svg)

![XXXXXXXXXXXXXX\label{figB}.](figs/RatQuad-B.svg)

![XXXXXXXXXXXXXX\label{figD}.](figs/RatQuad-D.svg)

![XXXXXXXXXXXXXX\label{figE}.](figs/RatQuad-E.svg)

![XXXXXXXXXXXXXX\label{figF}.](figs/RatQuad-F.svg)

![XXXXXXXXXXXXXX\label{figG}.](figs/RatQuad-G.svg)


--------------------------------------------------------------------------------
Arc type                           Angle range                         $\beta$       $\rho$         $\alpha$
---------------------------------  -------------                       -----------   -----------    -----------
Parabola                           $-\epsilon^\circ,\epsilon^\circ$    $0$           $1$            $1$

Arc less than semicircle                                               $(0,2)$       $(0,1)$        $(1/2,1)$

Semicircle                         $-90^\circ,90^\circ$                $2$           $0$            $1/2$

Arc greater than semicircle                                            $(2,\infty)$  $(-1,0)$       $(0,1/2)$

Full circle / indeterminate        $-180^\circ,180^\circ$              $\infty$      $-1$           $0$
--------------------------------------------------------------------------------

Table: CCCC brevity.\label{tabH}

![XXXXXXXXXXXXXX\label{figI}.](figs/RatQuad-I.svg)

![XXXXXXXXXXXXXX\label{figJ}.](figs/RatQuad-J.svg)

![XXXXXXXXXXXXXX\label{figK}.](figs/RatQuad-K.svg)


------------------------------------------------------------------------------------------------------
Vertegram face style             $\tau=$                                 $\tau=$
-------------------------------  ----------------------------            ----------------------------
Linear-tangent                   $u$                                     $\tan(\phi)$

Circular-tan-half                $\tan(\tfrac12\tan^{-1}(u))$            $\tan(\phi/2)$

Sin-tan-half                     $\tan(\tfrac12\sin^{-1}(u))$            $\tan(\tfrac12\sin^{-1}(u\tan(\phi)))$
------------------------------------------------------------------------------------------------------

Table: CCCC brevity.\label{tabL}


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
\sigma = \sqrt\frac{w_A}{w_C} \label{Eq67}
$$


Equation \ref{Eq68}:Eq68.

$$
\left|p_m-p_3\right| = \left|p_m-p_0\right| \label{Eq68}
$$

Equation \ref{Eq69}:Eq69.

$$
\rho = \frac{w_B}{\sqrt{w_Aw_C}} \label{Eq69}
$$

Equation \ref{Eq70}:Eq70.

$$
p_q = \rho p_m \label{Eq70}
$$

Equation \ref{Eq71}:Eq71.

$$
p_q = \left(\frac{1}{\sqrt{w_Aw_C}}\right) p_n \label{Eq71}
$$

Equation \ref{Eq72}:Eq72.

$$
p_q = \left(\frac23\right)\frac{\left|d\right|^2}{\sqrt{(a\cdot n)(b\cdot n)}} \label{Eq72}
$$

Equation \ref{Eq73}:Eq73.

$$
f_\rho(t) = \frac{%
\Bigl(\frac12-t\Bigr)^2p_0 + 2\Bigl(\frac12-t\Bigr)\Bigl(\frac12+t\Bigr)p_q + \Bigl(\frac12+t\Bigr)^2p_3
}{%
\Bigl(\frac12-t\Bigr)^2 + 2\rho\Bigl(\frac12-t\Bigr)\Bigl(\frac12+t\Bigr) + \Bigl(\frac12+t\Bigr)^2
} \label{Eq73}
$$

Equation \ref{Eq74}:Eq74.

$$
\alpha = \frac12(1+\rho) \label{Eq74}
$$

Equation \ref{Eq75}:Eq75.

$$
\alpha\beta^2 = \frac12(1-\rho) \label{Eq75}
$$

Equation \ref{Eq76}:Eq76.

$$
f_\rho(t) = \frac{%
\frac14\bigl(p_0+2p_q+p_3) + t\bigl(p_3-p_0\bigr) + t^2\bigl(p_o-2p_q+p_3\bigr)
}{%
\alpha(1+\beta^2t^2)
} \label{Eq76}
$$

Equation \ref{Eq77}:Eq77.

$$
p_a = \frac1{8\alpha}\Bigl(p_0+2p_q+p_3\Bigr) + \frac1{2\alpha\beta^2}\Bigl(p_0-2p_q+p_3\Bigr) \label{Eq77}
$$


Equation \ref{Eq78}:Eq78.

$$
p_c = \frac1{8\alpha}\Bigl(p_0+2p_q+p_3\Bigr) - \frac1{2\alpha\beta^2}\Bigl(p_0-2p_q+p_3\Bigr) \label{Eq78}
$$

Equation \ref{Eq79}:Eq79.

$$
p_s(t) = \frac1{2\alpha\beta}\Bigl(p_3-p_0\Bigr) \label{Eq79}
$$

Equation \ref{Eq80}:Eq80.

$$
f_\rho(t) = p_a + \left(\frac{2\beta t}{1+\beta^2 t^2}\right)p_s + \left(\frac{1-\beta^2 t^2}{1+\beta^2 t^2}\right)p_c \label{Eq80}
$$

Equation \ref{Eq81}:Eq81.

$$
\left[\, -2\tan^{-1}\left(\frac\beta2\right),\,\, 2\tan^{-1}\left(\frac\beta2\right)\, \right] \label{Eq81}
$$

Equation \ref{Eq82}:Eq82.

$$
(a\cdot n)(b\cdot n) > 0 \label{Eq82}
$$

Equation \ref{Eq83}:Eq83.

$$
\lambda\mu > 0 \label{Eq83}
$$

Equation \ref{Eq84}:Eq84.

$$
w_Aw_C > 0 \label{Eq84}
$$

Equation \ref{Eq85}:Eq85.

$$
\frac{(a\cdot n)(b\cdot n)}{\left|d\right|^2} \label{Eq85}
$$

Equation \ref{Eq86}:Eq86.

$$
-1 < \rho < 1 \label{Eq86}
$$

Equation \ref{Eq87}:Eq87.

$$
w_B = \sqrt{w_Aw_C} \label{Eq87}
$$

Equation \ref{Eq88}:Eq88.

$$
w_B = \left(\frac32\right) \frac{\sqrt{(a\cdot n)(b\cdot n)}}{\left|d\right|^2} \label{Eq88}
$$

Equation \ref{Eq89}:Eq89.

$$
f_\alpha(t) = (1-t)^3p_0 + 3t(1-t)^2\left(\frac23p_q + \frac13p_0\right) + 3t^2(1-t)\left(\frac23p_q + \frac13p_3\right) + t^3p_3 \label{Eq89}
$$


Equation \ref{Eq90}:Eq90.

$$
\lambda = \sigma\sqrt{\frac29\Bigl(1-\cos(\Omega)\Bigr)} = \frac{2w_B}{3w_A} \label{Eq90}
$$

Equation \ref{Eq91}:Eq91.

$$
\mu = \frac1\sigma\sqrt{\frac29\Bigl(1-\cos(\Omega)\Bigr)} = \frac{2w_B}{3w_C} \label{Eq91}
$$

Equation \ref{Eq92}:Eq92.

$$
w_A = \frac1\sigma \label{Eq92}
$$

Equation \ref{Eq93}:Eq93.

$$
w_B = \sqrt{\frac{1-\cos(\Omega)}{2}} = \rho \label{Eq93}
$$

Equation \ref{Eq94}:Eq94.

$$
w_C = \sigma \label{Eq94}
$$

Equation \ref{Eq95}:Eq95.

$$
XXX \label{Eq95}
$$

Equation \ref{Eq96}:Eq96.

$$
XXX \label{Eq96}
$$

Equation \ref{Eq97}:Eq97.

$$
XXX \label{Eq97}
$$


Equation \ref{Eq98}:Eq98.

$$
XXX \label{Eq98}
$$

Equation \ref{Eq99}:Eq99.

$$
XXX \label{Eq99}
$$


Equation \ref{Eq100}:Eq100.

$$
XXX \label{Eq100}
$$

Equation \ref{Eq101}:Eq101.

$$
XXX \label{Eq101}
$$

Equation \ref{Eq102}:Eq102.

$$
XXX \label{Eq102}
$$

Equation \ref{Eq103}:Eq103.

$$
XXX \label{Eq103}
$$

Equation \ref{Eq104}:Eq104.

$$
XXX \label{Eq104}
$$

Equation \ref{Eq105}:Eq105.

$$
XXX \label{Eq105}
$$

Equation \ref{Eq106}:Eq106.

$$
XXX \label{Eq106}
$$

Equation \ref{Eq107}:Eq107.

$$
XXX \label{Eq107}
$$


Equation \ref{Eq108}:Eq108.

$$
XXX \label{Eq108}
$$

Equation \ref{Eq109}:Eq109.

$$
XXX \label{Eq109}
$$
