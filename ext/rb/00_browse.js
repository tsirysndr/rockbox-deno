import { op_tree_get_entries } from "ext:core/ops";

function getEntries(path) {
  return op_tree_get_entries(path);
}

export default {
  tree: {
    getEntries,
  },
};
