import { op_tree_get_entries } from "ext:core/ops";

async function getEntries(path) {
  await op_tree_get_entries(path);
}

export default {
  tree: {
    getEntries,
  },
};
