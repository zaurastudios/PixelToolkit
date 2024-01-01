import { BrowserWindow, ipcMain, shell } from "electron";
import path from "path";
import fs from "fs";

export default function WorkspaceEventsHandler(mainWindow: BrowserWindow) {
  try {
    ipcMain.on("select-texture", (event, selectedPath) => {
      event.reply("selected-texture", selectedPath);
    });
  } catch (err) {
    console.error(err);
  }
}
