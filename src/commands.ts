import { core, window as tauriWindow } from "@tauri-apps/api";

export function toggleDevtools() {
  const label = tauriWindow.getCurrent().label;
  core.invoke<void>("toggle_devtools", { label });
}
