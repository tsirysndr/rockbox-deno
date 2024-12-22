import { op_get_global_settings, op_save_settings } from "ext:core/ops";

export async function getGlobalSettings() {
  await op_get_global_settings();
}

export async function saveSettings() {
  await op_save_settings();
}
