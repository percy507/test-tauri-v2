import "./app.less";

import { getCurrent, getAll } from "@tauri-apps/plugin-window";
import { toggleDevtools } from "./commands";

function App() {
  return (
    <div className="container">
      <button
        onClick={() => {
          let win = getCurrent();
          win.maximize();
        }}
      >
        maximize
      </button>
      <button onClick={toggleDevtools}>open devtools</button>
      <button
        onClick={() => {
          const wins = getAll();
          wins.find((el) => el.label === "win2")?.show();
        }}
      >
        Open second window
      </button>
    </div>
  );
}

export default App;
