import React, { useState } from "react";
import ReactDOM from "react-dom/client";
import axios from "axios";

function App() {
  const [repo, setRepo] = useState("");
  const [logs, setLogs] = useState([]);

  const trigger = async () => {
    await axios.post("http://localhost:3000/webhook/github", {
      repository: { full_name: repo },
      pull_request: { number: 1 }
    });

    setLogs([...logs, "Scan triggered for " + repo]);
  };

  return (
    <div style={{ padding: 40 }}>
      <h1>🚀 OmniUil Security Dashboard</h1>

      <input value={repo} onChange={e => setRepo(e.target.value)} placeholder="org/repo" />
      <button onClick={trigger}>Scan PR</button>

      <ul>
        {logs.map((l, i) => <li key={i}>{l}</li>)}
      </ul>
    </div>
  );
}

ReactDOM.createRoot(document.getElementById("root")).render(<App />);
