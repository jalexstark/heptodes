## Common rendering function.  Customize using config.
function success = render_auxiliary(auxiliary_links, output_name, config)
  [n m] = size(auxiliary_links);
  scaled_auxiliary_links = (auxiliary_links + 0.5) / n;
  locations = scaled_auxiliary_links(:, 1:3);

  figure('visible','off');
  plot(locations(:,1), locations(:,3), "ko", "markersize", 5*32/n, "markerfacecolor", [0 0 0.5]);
  set(gca, "xtick", [], "ytick", [], "looseinset", [0.02 0.02 0.02 0.02]);
  axis("square");

  hold("on");

  [tmp head] = min(auxiliary_links(:, 2));
  [tmp leftmost]= min(auxiliary_links(:, 1));
  [tmp rightmost] = max(auxiliary_links(:, 1));
  ## [tmp tail] = max(auxiliary_links(:, 2));

  if isfield(config, "nw")
    curr = head;
    while curr != leftmost
      curr = auxiliary_links(curr, 4) + 1;
    endwhile
    while auxiliary_links(curr, 4) >= 0
      plot(scaled_auxiliary_links(curr, [1 4]), scaled_auxiliary_links(curr, [3 6]), config.nw);
      old_curr = curr;
      curr = auxiliary_links(curr, 4) + 1;
      auxiliary_links(old_curr, 4) = -1;
    endwhile
  endif
  if isfield(config, "sw")
    curr = head;
    while curr != leftmost
      plot(scaled_auxiliary_links(curr, [1 4]), scaled_auxiliary_links(curr, [3 6]), config.sw);
      old_curr = curr;
      curr = auxiliary_links(curr, 4) + 1;
      auxiliary_links(old_curr, 4) = -1;
    endwhile
  endif

  if isfield(config, "ne")
    curr = head;
    while curr != rightmost
      curr = auxiliary_links(curr, 7) + 1;
    endwhile
    while auxiliary_links(curr, 7) >= 0
      plot(scaled_auxiliary_links(curr, [1 7]), scaled_auxiliary_links(curr, [3 9]), config.ne);
      old_curr = curr;
      curr = auxiliary_links(curr, 7) + 1;
      auxiliary_links(old_curr, 7) = -1;
    endwhile
  endif
  if isfield(config, "se")
    curr = head;
    while curr != rightmost
      plot(scaled_auxiliary_links(curr, [1 7]), scaled_auxiliary_links(curr, [3 9]), config.se);
      old_curr = curr;
      curr = auxiliary_links(curr, 7) + 1;
      auxiliary_links(old_curr, 7) = -1;
    endwhile
  endif

  if isfield(config, "secondary")
    for i = 1:n
      if auxiliary_links(i, 4) >= 0
	plot(scaled_auxiliary_links(i, [1 4]), scaled_auxiliary_links(i, [3 6]), config.secondary);
      endif
    endfor
  endif

  if (isfield(config, "tertiary"))
    for i = 1:n
      if auxiliary_links(i, 7) >= 0
	plot(scaled_auxiliary_links(i, [1 7]), scaled_auxiliary_links(i, [3 9]), config.tertiary);
      endif
    endfor
  endif

  print (output_name, "-dsvg", "-S240,240");

  hold("off");

  success = 0;
