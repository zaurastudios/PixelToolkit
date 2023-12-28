import { BrowserWindow, ipcMain } from "electron";
import fs from "fs";
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
        config.projectFiles.map((project) => {
          if (fs.existsSync(`${project.path}/pack.png`)) {
            project.packPng = `${project.path}/pack.png`;
            return project;
          }

          return project;
        });

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
  } catch (err) {
    console.error(err);
  }
}
