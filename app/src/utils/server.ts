import { invoke } from "@tauri-apps/api/core";

export const startServer = () => {
  invoke("start_server", { port: 1234 });
};

export const stopServer = async () => {
  const ip = await getIp();
  try {
    await fetch(`http://${ip}:1234/down?password=${8910}`);
  } catch (e) {
    console.log(e);
  }
};

export const getIp = async () => {
  return (await invoke("get_current_ip")) as String;
};
