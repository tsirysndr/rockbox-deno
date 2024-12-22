import { op_get_global_status, op_get_rockbox_version } from "ext:core/ops";

export async function getGlobalStatus() {
  await op_get_global_status();
}

export async function getRockboxVersion() {
  return await op_get_rockbox_version();
}
