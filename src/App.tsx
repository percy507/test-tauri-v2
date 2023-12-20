import "./app.less";

import { window as tauriWindow } from "@tauri-apps/api";
import { toggleDevtools } from "./commands";

function App() {
  return (
    <div className="container">
      <button
        onClick={() => {
          let win = tauriWindow.getCurrent();
          win.maximize();
        }}
      >
        maximize
      </button>
      <button onClick={toggleDevtools}>open devtools</button>
      <button
        onClick={() => {
          const wins = tauriWindow.getAll();
          wins.find((el) => el.label === "win2")?.show();
        }}
      >
        Open second window
      </button>
    </div>
  );
}

export default App;
