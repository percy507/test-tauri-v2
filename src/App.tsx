import "./app.less";

import { window as tauriWindow } from "@tauri-apps/api";
import { toggleDevtools } from "./commands";

export default function App() {
  return (
    <div>
      <div className="btns">
        <button onClick={toggleDevtools}>open devtools</button>
        <button
          onClick={() => {
            const win = tauriWindow.getCurrent();
            win.close();
          }}
        >
          close window
        </button>
      </div>
    </div>
  );
}
