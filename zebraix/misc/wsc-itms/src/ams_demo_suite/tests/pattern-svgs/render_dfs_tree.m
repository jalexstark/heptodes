## Usage:
##
## octave [--silent] --eval "auxiliary_links=[INPUT_SCRIPT]; render_dfs_tree(auxiliary_links, [OUTPUT_FILE]);"
function success = render_dfs_tree(auxiliary_links, output_name)
  config.secondary = "r-";
  config.sw = "b-";
  success = render_auxiliary(auxiliary_links, output_name, config);
  return

