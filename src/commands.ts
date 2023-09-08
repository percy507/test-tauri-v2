import { getCurrent } from "@tauri-apps/plugin-window";
import { invoke } from "@tauri-apps/api";

export function toggleDevtools() {
  const label = getCurrent().label;
  invoke<void>("toggle_devtools", { label });
}
