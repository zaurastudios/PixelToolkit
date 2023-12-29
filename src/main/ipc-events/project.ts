import { BrowserWindow, ipcMain, shell } from "electron";
import { getConfigData, getProjectData, removeProject } from "../config-dir";

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

    ipcMain.on("open-in-folder", (event, shellPath) => {
      try {
        shell.openPath(shellPath);
      } catch (err) {
        console.error(err);
      }
    });

    ipcMain.on("delete-project", (event, id) => {
      try {
        removeProject(id);
        event.reply("deleted-project", {
          redirect: "/",
          toast: "Deleted project.",
        });
      } catch (err) {
        console.err(err);
        event.reply("deleted-project", {
          redirect: "/",
          toast: "Unable to delete.",
          error: true,
        });
      }
    });
  } catch (err) {
    console.error(err);
  }
}
