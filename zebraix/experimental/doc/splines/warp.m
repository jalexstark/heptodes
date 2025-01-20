## usage: t = warp(t, gamma, a = 0.0, b = 1.0)
##
## Bilinear Warping, applied over range (a, b).
##
## For presentation / documentation, cleanest to show (0, r) or (-r, r).
##
## b > a.
## Default range is (0, 1).
## t can be matrix of any size.
function t = warp (t, gamma, a = 0.0, b = 1.0)
  ## s = b - a;
  ## t = (t - a) / s;
  ## t = (1 + gamma) * t ./ ( 2 * gamma * t + 1 - gamma );
  ## t = t * s + a;

  t = ((1 + gamma) * t * b - (1 - gamma) *t * a - 2 * gamma * a * b) ./ ( 2 * gamma * t - (1 + gamma) * a + (1 - gamma) * b );

  ## s = b - a;
  ## m = (a + b) / 2;
  ## t = ((1 + gamma) * t  + (m - s / 2) * gamma * (   2 * t - 2 * m  -s )) ./ ( 2 * gamma * t - 2 * gamma * m +  s );
endfunction

return

## Test example
##
## Note that second set of points ("x") are exchanged.
t = 2 + (0:0.01:1)';
tp = 2 + (0:0.1:1)';
plot(t, warp(t, 0.5, 2, 3), t, warp(t, -0.5, 2, 3), tp, warp(tp, 0.5, 2, 3), "o", warp(tp, 0.5, 2, 3), tp, "x")
axis("square")
