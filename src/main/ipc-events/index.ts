import { ipcMain, BrowserWindow } from "electron";
import CreateEventsHandlers from "./create";

export default function AllEvents(mainWindow: BrowserWindow | null) {
  if (mainWindow) {
    CreateEventsHandlers(mainWindow);

    // Close app event
    ipcMain.on("close-app", () => app.quit());
  }
}
