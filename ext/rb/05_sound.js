import { op_adjust_volume } from "ext:core/ops";

export async function adjustVolume(steps) {
  await op_adjust_volume(steps);
}
