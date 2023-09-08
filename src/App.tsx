import "./app.less";

import { getCurrent } from "@tauri-apps/plugin-window";
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
    </div>
  );
}

export default App;
