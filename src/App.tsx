import "./app.less";

import { toggleDevtools } from "./commands";
import { useState } from "react";

const isWindows =
  ["Win32", "Win64", "Windows", "WinCE"].indexOf(navigator.platform) !== -1;

export default function App() {
  return (
    <div>
      {isWindows && (
        <div className="titlebar" data-tauri-drag-region>
          title
        </div>
      )}

      <div className="content">
        <div className="time-now">{new Date().toISOString()}</div>
        <div className="btns">
          <button onClick={toggleDevtools}>open devtools</button>
        </div>

        <TestCase />
      </div>
    </div>
  );
}

function TestCase() {
  const [mode, setMode] = useState("textarea");

  const renderEditor = () => {
    const defaultValue = `The form element that the <textarea> element is associated with (its "form owner"). The value of the attribute must be the id of a form element in the same document. If this attribute is not specified, the <textarea> element must be a descendant of a form element. This attribute enables you to place <textarea> elements anywhere within a document, not just as descendants of form elements.`;

    if (mode === "textarea") {
      return (
        <textarea
          style={{ display: "block", width: 300 }}
          rows={10}
          defaultValue={defaultValue}
        />
      );
    }

    if (mode === "contenteditable") {
      return <div contentEditable>{defaultValue}</div>;
    }

    let url = "";
    if (mode === "codemirror") {
      url = "https://codemirror.net/";
    } else if (mode === "prosemirror") {
      url = "https://prosemirror.net/";
    } else if (mode === "slate") {
      url = "https://www.slatejs.org/examples/richtext";
    } else if (mode === "monaco") {
      url =
        "https://microsoft.github.io/monaco-editor/playground.html?source=v0.50.0#example-creating-the-editor-hello-world";
    } else if (mode === "tinymce") {
      url = "https://fiddle.tiny.cloud/";
    } else if (mode === "quill") {
      url = "https://quilljs.com/";
    }

    return (
      <iframe style={{ width: "100%", height: 600, border: 0 }} src={url} />
    );
  };

  return (
    <div className="testCase">
      <div className="testCase-title">Test undo & redo</div>
      <div style={{ marginBottom: 16 }}>
        <select value={mode} onChange={(e) => setMode(e.target.value)}>
          <option value="textarea">[html]textarea</option>
          <option value="contenteditable">[html]contenteditable</option>
          <option value="codemirror">codemirror</option>
          <option value="prosemirror">prosemirror</option>
          <option value="slate">slate editor</option>
          <option value="monaco">Monaco Editor</option>
          <option value="tinymce">tinymce editor</option>
          <option value="quill">quill editor</option>
        </select>
      </div>
      <div
        className="testCase-content"
        style={{ border: "1px dashed gray", padding: 10 }}
      >
        {renderEditor()}
      </div>
    </div>
  );
}
