import "./App.css";
import { startServer, stopServer } from "./utils/server";

function App() {
  "wswswww";
  return (
    <div className="container">
      <button onClick={() => startServer()}>Start Server</button>
      <button onClick={() => stopServer()}>Stop Server</button>
    </div>
  );
}

export default App;
