## Usage:
##
## octave [--silent] --eval "auxiliary_links=[INPUT_SCRIPT]; render_lozenge(auxiliary_links, [OUTPUT_FILE]);"
function success = render_lozenge(auxiliary_links, output_name)
  config.se = "r-";
  config.nw = "r-";
  config.sw = "b-";
  config.ne = "b-";
  success =  render_auxiliary(auxiliary_links, output_name, config);

  ## fontsize does not work.
  xh = xlabel("Indices");
  set(xh, "position", get(xh,"position") - [0, 0.04, 0]);
  yh = ylabel("Values");
  set(yh, "position", get(yh,"position") - [0.06, 0, 0]);

  annotation_color = [0.05, 0.4, 0.05];
  text(0.32, -0.04, 0, "S head", "horizontalalignment", "center", "color", annotation_color);
  text(0.48, 1.04, 0, "N tail", "horizontalalignment", "center", "color", annotation_color);
  text(-0.04, 0.33, 0, "W anchor", "horizontalalignment", "center", "color", annotation_color, "rotation", 90);
  text(1.04, 0.68, 0, "E anchor", "horizontalalignment", "center", "color", annotation_color, "rotation", 90);

  set(gca, "looseinset", get(gca, "looseinset") + [0, 0, 0.06, 0])

  print (output_name, "-dsvg", "-S240,240", "-F:24");

  return
