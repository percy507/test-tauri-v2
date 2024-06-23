import "./app.less";

import { window as tauriWindow } from "@tauri-apps/api";
import { toggleDevtools } from "./commands";
import { useEffect, useState } from "react";

const isWindows =
  ["Win32", "Win64", "Windows", "WinCE"].indexOf(navigator.platform) !== -1;

export default function App() {
  const [title, setTitle] = useState("");
  const win = tauriWindow.getCurrent();

  useEffect(() => {
    win.title().then(setTitle);
  }, [win]);

  return (
    <div className="app-root">
      <div className="time-now">{new Date().toISOString()}</div>

      {isWindows && (
        <div className="titlebar" data-tauri-drag-region>
          {title}
        </div>
      )}

      <div className="btns">
        <button onClick={toggleDevtools}>open devtools</button>
      </div>

      <TestCase />
    </div>
  );
}

function TestCase() {
  return (
    <div className="testCase">
      <div className="testCase-title">Demo</div>
      <div className="testCase-content">content</div>
    </div>
  );
}
