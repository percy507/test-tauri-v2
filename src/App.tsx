import "./app.less";

import { window as tauriWindow } from "@tauri-apps/api";
import { toggleDevtools } from "./commands";

export default function App() {
  const win = tauriWindow.getCurrent();

  return (
    <div>
      <h3>Current window label: {win.label}</h3>
      <div className="btns">
        <button onClick={toggleDevtools}>open devtools</button>
        <button onClick={() => win.close()}>close window</button>
        <button onClick={() => createWindow()}>create window</button>
      </div>
    </div>
  );
}

function createWindow() {
  new tauriWindow.Window("win2", {
    url: "http://localhost:1420",
    title: "win-title2",
    width: 300,
    height: 300,
  });
}
