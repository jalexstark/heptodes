## usage: success = render_pattern (input_name, output_name)
##
## octave [--silent] --eval "render_pattern('${input_name}', '${output_name}')"
function success = render_pattern(input_name, output_name)
  pattern = csvread(input_name);
  [n m] = size(pattern);
  locations = (pattern + 0.5) / n;

  figure('visible','off');
  plot(locations(:,1), locations(:,2), "ko", "markersize", 5*32/n, "markerfacecolor", [0 0 0.5]);
  set(gca, "xtick", [], "ytick", [], "looseinset", [0.02 0.02 0.02 0.02]);
  axis("square");

  print (output_name, "-dsvg", "-S240,240");
  success = 0;
endfunction
