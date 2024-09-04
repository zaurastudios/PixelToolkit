import { appDataDir } from "@tauri-apps/api/path";
import { platform } from "@tauri-apps/plugin-os";

export async function getPlatform() {
  return await platform();
}

export async function getConfigDir() {
  return await appDataDir();
}
