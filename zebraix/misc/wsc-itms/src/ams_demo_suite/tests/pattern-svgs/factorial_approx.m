function approximation = factorial_approx(n)
   approximation = n*log(n) - n + (log(n*(1+4*n*(1+2*n))))/6 + log(pi)/2;
endfunction
