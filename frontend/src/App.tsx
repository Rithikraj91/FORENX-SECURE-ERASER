import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [targetPath, setTargetPath] = useState("");
  const [driveId, setDriveId] = useState("");
  const [status, setStatus] = useState<string>("Ready");
  const [output, setOutput] = useState<string>("");

  const handleDriveErasure = async () => {
    setStatus("Executing Mock Drive Erasure...");
    try {
      const response = await invoke("run_mock_drive_erasure", {
        driveId: driveId || "\\\\.\\PhysicalDrive0",
      });
      setStatus("Completed successfully");
      setOutput(JSON.stringify(response, null, 2));
    } catch (err) {
      setStatus("Error running drive erasure");
      setOutput(String(err));
    }
  };

  const handleFileErasure = async () => {
    setStatus("Executing Mock File Erasure...");
    try {
      const response = await invoke("run_mock_file_erasure", {
        path: targetPath || "C:\\test\\sample.txt",
      });
      setStatus("Completed successfully");
      setOutput(JSON.stringify(response, null, 2));
    } catch (err) {
      setStatus("Error running file erasure");
      setOutput(String(err));
    }
  };

  return (
    <div style={{ padding: "2rem", fontFamily: "sans-serif", maxWidth: "700px", margin: "0 auto" }}>
      <h2>Forenx Secure Eraser - Step 7 Mock UI</h2>
      <p>Frontend Tauri IPC Harness (Simulation Only)</p>

      <div style={{ border: "1px solid #ccc", padding: "1rem", borderRadius: "8px", marginBottom: "1rem" }}>
        <h3>File Eraser (Mock)</h3>
        <input
          type="text"
          placeholder="Target file path (e.g. C:\test\mock.txt)"
          value={targetPath}
          onChange={(e) => setTargetPath(e.target.value)}
          style={{ width: "80%", padding: "8px", marginRight: "8px" }}
        />
        <button onClick={handleFileErasure} style={{ padding: "8px 16px" }}>
          Run Mock File Erasure
        </button>
      </div>

      <div style={{ border: "1px solid #ccc", padding: "1rem", borderRadius: "8px", marginBottom: "1rem" }}>
        <h3>Drive Eraser (Mock)</h3>
        <input
          type="text"
          placeholder="Target Drive ID (e.g. \\.\PhysicalDrive0)"
          value={driveId}
          onChange={(e) => setDriveId(e.target.value)}
          style={{ width: "80%", padding: "8px", marginRight: "8px" }}
        />
        <button onClick={handleDriveErasure} style={{ padding: "8px 16px" }}>
          Run Mock Drive Erasure
        </button>
      </div>

      <div style={{ marginTop: "1rem", background: "#f4f4f4", padding: "1rem", borderRadius: "8px" }}>
        <h4>Status: {status}</h4>
        <pre style={{ whiteSpace: "pre-wrap", wordBreak: "break-all" }}>{output}</pre>
      </div>
    </div>
  );
}

export default App;