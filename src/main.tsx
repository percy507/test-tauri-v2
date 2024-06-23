import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import hotkeys from "hotkeys-js";

hotkeys("command+r,ctrl+r", (e) => {
  e.preventDefault(), e.stopPropagation();
  location.reload();
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
