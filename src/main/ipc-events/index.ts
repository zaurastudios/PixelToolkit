import { ipcMain, BrowserWindow, app } from "electron";
import CreateEventsHandlers from "./create";
import ProjectEventsHandler from "./project";
import WorkspaceEventsHandler from "./workspace";

export default function AllEvents(mainWindow: BrowserWindow | null) {
  if (mainWindow) {
    CreateEventsHandlers(mainWindow);
    ProjectEventsHandler(mainWindow);
    WorkspaceEventsHandler(mainWindow);

    // Close app event
    ipcMain.on("close-app", () => app.quit());

    // Setting the title
    ipcMain.on("set-title", (event, title) => mainWindow.setTitle(title));
  }
}
