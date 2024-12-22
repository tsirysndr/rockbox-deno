import { op_get_global_status, op_get_rockbox_version } from "ext:core/ops";

export function getGlobalStatus() {
  return op_get_global_status();
}

export function getRockboxVersion() {
  return op_get_rockbox_version();
}
