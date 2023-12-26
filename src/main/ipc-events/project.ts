import { BrowserWindow, ipcMain } from "electron";
import { Config, getConfigData } from "../config-dir";

export default function ProjectEventsHandler(mainWindow: BrowserWindow) {
  try {
    ipcMain.on("get-my-projects", (event) => {
      const config: Config | null = getConfigData();
      if (config) {
        event.reply("my-projects", config);
      }
    });
  } catch (err) {
    console.error(err);
  }
}
