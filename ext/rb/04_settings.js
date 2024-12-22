import { op_get_global_settings, op_save_settings } from "ext:core/ops";

export function getGlobalSettings() {
  return op_get_global_settings();
}

export function saveSettings() {
  return op_save_settings();
}
