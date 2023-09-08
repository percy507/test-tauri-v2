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

      <div style={{ padding: 24, border: "1px solid red" }}>
        <div contentEditable>
          <p>This is a paragraph.</p>
        </div>
      </div>
    </div>
  );
}

export default App;
