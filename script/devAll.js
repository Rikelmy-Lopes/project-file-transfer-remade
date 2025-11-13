import { spawn } from "child_process";
import { watch } from "fs";
const DELAY = 500;
const CWD = process.cwd() + "/app";

let child;
let timeout;

spawn("npm", ["run", "tauri", "dev"], { shell: true, stdio: "inherit", cwd: CWD });

watch("./app/src", { recursive: true }, () => {
  clearTimeout(timeout);

  timeout = setTimeout(() => {
    if (child && !child.killed) child.kill();
    child = spawn("npm", ["run", "build:no-typecheck"], { shell: true, stdio: "inherit", cwd: CWD });
  }, DELAY);
});
