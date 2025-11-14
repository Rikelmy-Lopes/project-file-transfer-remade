import { useState } from "react";
import "./App.css";
import { startServer, stopServer } from "./utils/server";

function App() {
  const [isServerRunning, setIsServerRunning] = useState(false);

  return (
    <div className="container">
      <button
        onClick={() => {
          startServer();
          setIsServerRunning(true);
        }}
        disabled={isServerRunning}
      >
        Start Server
      </button>
      <button
        onClick={() => {
          stopServer();
          setIsServerRunning(false);
        }}
        disabled={!isServerRunning}
      >
        Stop Server
      </button>
    </div>
  );
}

export default App;
