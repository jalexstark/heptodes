## usage: t = sinetan (t, range, alpha, beta = 0.0)
##
## Sine-Tangent warping-like transformation. Not quite the same as the
## bilinear warp, in that this is not readily invertible.
function t = sinetan(t, range, alpha, beta = 0.0, scaling = "range")
  if (length(alpha) > 1)
    [m, n] = size(t);
    assert(length(beta) == length(alpha), "For multiple warps, length of beta must match that of alpha.");
    assert(n == length(alpha), "For multiple warps, number of columns of t must match that of alpha.");
    for i = 1:n
      t(:, i) = sinetan(t(:, i), range, alpha(i), beta(i), scaling);
    endfor
  else
    ## (-0.5, 0.5) Second attempt
    ## Incorrect: beta_mod = beta * (1 + alpha * range * range) / (1 - beta * (1 + alpha * range * range));
    ## beta_mod = beta*range*(1-alpha*range*range) / (1+alpha*range*range);
    ## range = 0.5
    ## -3.95 -> 79.5  -3.9 -> 39.5  -2 -> 3*range  -1 -> 0.81  0 -> range  1 -> 0.3  2 -> 0.2* range  1/r/r -> 0
    ## range = 0.75
    ## -1/r/r * 0.995 -> 300 ish  0.5/r/r -> 0.28*range
    ## range = 1.0, range = 2.0
    ## -1/r/r * 0.995 -> 100+  -0.5/r/r -> 3*range  0.5/r/r -> 0.333333*range
    ## alpha_factor = (1 + alpha * range * range);
    ## t = range * (beta_mod * t .* t / range / range + 2 * t - beta_mod) ./ ( 1 + alpha * t .* t ) * alpha_factor / (beta_mod + 2 * range - beta_mod);
    t = (beta*(1-alpha*range*range) * t .* t / range + 2 * t * (1+alpha*range*range) - beta*range*(1-alpha*range*range)) ./ ( 1 + alpha * t .* t ) / 2;
    if scaling == "trig"
      t = t * 2 / (1+alpha*range*range)
    else
      assert(scaling == "range", "Argument <scaling> must be either \"range\" or \"trig\"");
    endif
    ## (-1.0, 1.0)
    ## t = (beta * alpha * t .* t + 2 * t - beta * alpha) ./ ((1 - alpha) * t .* t + 1 + alpha);
    ## (-0.5, 0.5) First attempt
    ## t = (2 * beta * alpha * t .* t + 2 * t - beta * alpha / 2) ./ (4 * ( 1 - alpha) * t .* t + 1 + alpha);
    ## (-0.5, 0.5) First attempt
    ## t = (4 * beta * t .* t + 2 * t - beta) ./ (4 * alpha * t .* t + 2 - alpha);
  endif
endfunction

return

## (-0.5, 0.5) Second attempt

range = 0.5;
limit = 1 / range / range;
t = (-1:0.002:1)' * range;
tp = (-1:0.2:1)' * range;
plot(t * ones(1,9), sinetan(t * ones(1, 9), range, [(-1:0.3333:1)*limit / 4 limit -0.97 * limit], 0 * ones(1,9)), ...
     tp * ones(1,9), sinetan(tp * ones(1, 9), range, [(-1:0.3333:1)*limit / 4 limit -0.97 * limit], 0 * ones(1,9)), "o"); axis("square", "tight")
plot(t * ones(1,9), sinetan(t * ones(1, 9), range, (-0.97:1.97/8:1.01) * limit, 1 * ones(1,9)), ...
     tp * ones(1,9), sinetan(tp * ones(1, 9), range, (-0.97:1.97/8:1.01) * limit, 1 * ones(1,9)), "o"); axis("square", "tight")

plot(t * ones(1,7), sinetan(t * ones(1, 7), range, limit / 4 * ones(1,7), (-1:0.3333:1))); axis("square", "tight")

plot(t * ones(1,7), sinetan(t * ones(1, 7), range, 0.1 * limit / 4 * ones(1,7), (-1:0.333333333:1)), tp * ones(1,7), sinetan(tp * ones(1, 7), range, 0.1 * limit / 4 * ones(1,7), (-1:0.333333333:1)), "o"); axis("square", "tight")



## (-0.5, 0.5) First attempt

t = (-0.5:0.01:0.5)';
tp = (-0.5:0.1:0.5)';
plot(t * ones(1,7), sinetan(t * ones(1, 7), (0:6), 0 * ones(1,7)), tp * ones(1,7), sinetan(tp * ones(1, 7), (0:6), 0 * ones(1,7)), "o"); axis("square", "tight")

plot(t * ones(1,7), sinetan(t * ones(1, 7), 0.1 * ones(1,7), (-1:0.333333333:1)), tp * ones(1,7), sinetan(tp * ones(1, 7), 0.1 * ones(1,7), (-1:0.333333333:1)), "o"); axis("square", "tight")

plot(t * ones(1,7), sinetan(t * ones(1, 7), 4 * ones(1,7), (-1:0.333333333:1)), tp * ones(1,7), sinetan(tp * ones(1, 7), 4 * ones(1,7), (-1:0.333333333:1)), "o"); axis("square", "tight")

th = tan(0.5 * atan(tp * 2)) * 0.5 / (sqrt(2) - 1);
plot(th * ones(1,7), sinetan(th * ones(1, 7), (0:6), 0 * ones(1,7)), "o", t, 0.5*sin(pi*t), "--"); axis("square", "tight")

sinetan(th, 1.414, 0)

## (-1.0, 1.0), and simpler th.

t = (-1.0:0.02:1.0)';
tp = (-1.0:0.2:1.0)';
th = tan(0.5 * atan(tp));

plot(t * ones(1,7), sinetan(t * ones(1, 7), (0:6), 0 * ones(1,7)), tp * ones(1,7), sinetan(tp * ones(1, 7), (0:6), 0 * ones(1,7)), "o"); axis("square", "tight")

plot(t * ones(1,7), sinetan(t * ones(1, 7), 0.1 * ones(1,7), (-1:0.333333333:1)), tp * ones(1,7), sinetan(tp * ones(1, 7), 0.1 * ones(1,7), (-1:0.333333333:1)), "o"); axis("square", "tight")

plot(t * ones(1,7), sinetan(t * ones(1, 7), 4 * ones(1,7), (-1:0.333333333:1)), tp * ones(1,7), sinetan(tp * ones(1, 7), 4 * ones(1,7), (-1:0.333333333:1)), "o"); axis("square", "tight")

plot(th * ones(1,7), sinetan(th * ones(1, 7), (0:6), 0 * ones(1,7)), "o", t, sin(pi*t), "--"); axis("square", "tight")
