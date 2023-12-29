import { BrowserWindow, ipcMain, shell } from "electron";
import { getConfigData, getProjectData } from "../config-dir";

export default function ProjectEventsHandler(mainWindow: BrowserWindow) {
  try {
    ipcMain.on("get-my-projects", (event) => {
      const config = getConfigData();
      if (config) {
        config.projectFiles.sort(
          (a, b) =>
            new Date(b.dateModified).getTime() -
            new Date(a.dateModified).getTime(),
        );

        event.reply("my-projects", config);
      }
    });

    ipcMain.on("get-project-data-with-id", (event, id) => {
      const projectFile = getProjectData(id);
      if (!projectFile) {
        event.reply("project-data-reply", false);
        return;
      }

      event.reply("project-data-reply", projectFile);
    });

    ipcMain.on("open-in-folder", async (event, shellPath) => {
      shell.openPath(shellPath);
      try {
        shell.openPath(shellPath);
      } catch (err) {
        console.error(err);
      }
    });
  } catch (err) {
    console.error(err);
  }
}
