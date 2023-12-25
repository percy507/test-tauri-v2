import * as fs from "@tauri-apps/plugin-fs";

export function TestFS() {
  return (
    <div className="btns">
      <button
        onClick={() => {
          fs.writeTextFile("/Users/m2/Downloads/111.txt", "111_data")
            .then(() => console.log("success"))
            .catch((err) => console.log(`fail: ${err.message}`));
        }}
      >
        writeTextFile (absolute path)
      </button>
      <button
        onClick={() => {
          fs.writeTextFile("222.txt", "222_data", {
            baseDir: fs.BaseDirectory.Download,
          })
            .then(() => console.log("success"))
            .catch((err) => console.log(`fail: ${err.message}`));
        }}
      >
        writeTextFile (baseDir + relative path)
      </button>

      <button
        onClick={() => {
          let data = new TextEncoder().encode("111_bin_data");
          fs.writeFile("/Users/m2/Downloads/111.bin.txt", data)
            .then(() => console.log("success"))
            .catch((err) => console.log(`fail: ${err.message}`));
        }}
      >
        writeFile (absolute path)
      </button>
      <button
        onClick={() => {
          let data = new TextEncoder().encode("222_bin_data");
          fs.writeFile("222.bin.txt", data, {
            baseDir: fs.BaseDirectory.Download,
          })
            .then(() => console.log("success"))
            .catch((err) => console.log(`fail: ${err.message}`));
        }}
      >
        writeFile (baseDir + relative path)
      </button>
    </div>
  );
}
