## usage: t = cubicline (t, range, alpha, beta)
##
## Pseudo-warp series in range (-r, r) as if a cubic spline.
function t = cubicline (t, range, alpha, beta)
  if (length(alpha) > 1)
    [m, n] = size(t);
    assert(length(beta) == length(alpha), "For multiple warps, length of beta must match that of alpha.");
    assert(n == length(alpha), "For multiple warps, number of columns of t must match that of alpha.");
    for i = 1:n
      t(:, i) = cubicline(t(:, i), range, alpha(i), beta(i));
    endfor
  else
    r = range;
    alpha_mod = 1/3 - alpha;
    if alpha < 0
      phi = sqrt((1 - 3* alpha) * (1 + alpha));
    else
      phi = 1 - alpha;
    endif
    p = r * (phi * beta - alpha);
    q = r * (phi * beta + alpha);
    t = (-r * (r-t).^3 + 3 * p * (r-t).^2.*(r+t) + 3 * q * (r-t).*(r+t).^2 + r * (r+t).^3) / r^3 / 8;
    ## s = sqrt( beta*beta-(4-3*alpha_mod)*alpha_mod);
    ## r * (beta + [-s s]) /(3*alpha_mod)
    ## (2*r+3*p-3*q) - (6*r*alpha_mod)
  endif
endfunction


return


range = 2.0;
t = (-1:0.002:1)' * range;
tp = (-1:0.2:1)' * range;

plot(t, cubicline(t, range, 0, 0), tp, cubicline(tp, range, 0, 0), "o"); axis("square", "tight")
plot(t, cubicline(t, range, 0.01,2/3+0.01), tp, cubicline(tp, range, 0.01,2/3+0.01), "o"); axis("square", "tight");grid


plot(t * ones(1,7), cubicline(t * ones(1, 7), range, (-1:0.3333:1), 0 * ones(1,7)), tp * ones(1,7), cubicline(tp * ones(1, 7), range, (-1:0.3333:1), 0 * ones(1,7)), "o"); axis("square", "tight")
plot(t * ones(1,7), cubicline(t * ones(1, 7), range, (-1:0.3333:1), 1 * ones(1,7)), tp * ones(1,7), cubicline(tp * ones(1, 7), range, (-1:0.3333:1), 1 * ones(1,7)), "o"); axis("square", "tight")


plot(t * ones(1,7), cubicline(t * ones(1, 7), range, 0.3 * ones(1,7), (-1:0.333333333:1)), tp * ones(1,7), cubicline(tp * ones(1, 7), range, 0.3 * ones(1,7), (-1:0.333333333:1)), "o"); axis("square", "tight")
plot(t * ones(1,7), cubicline(t * ones(1, 7), range, -0.5 * ones(1,7), (-1:0.333333333:1)), tp * ones(1,7), cubicline(tp * ones(1, 7), range, -0.5 * ones(1,7), (-1:0.333333333:1)), "o"); axis("square", "tight")
