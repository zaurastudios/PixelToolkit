import { ipcMain, BrowserWindow, app } from "electron";
import CreateEventsHandlers from "./create";
import ProjectEventsHandler from "./project";

export default function AllEvents(mainWindow: BrowserWindow | null) {
  if (mainWindow) {
    CreateEventsHandlers(mainWindow);
    ProjectEventsHandler(mainWindow);

    // Close app event
    ipcMain.on("close-app", () => app.quit());

    // Setting the title
    ipcMain.on("set-title", (event, title) => mainWindow.setTitle(title));
  }
}
