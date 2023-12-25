import { randomUUID } from "crypto";
import { BrowserWindow, dialog, ipcMain } from "electron";
import fs from "fs";
import {
  Config,
  ProjectFile,
  getConfigData,
  saveConfigData,
} from "../config-dir";

export default function CreateEventsHandlers(mainWindow: BrowserWindow) {
  // Open directory dialog
  ipcMain.on("open-directory", async (e) => {
    const dir = await dialog.showOpenDialog(mainWindow!, {
      properties: ["openDirectory"],
    });

    const { filePaths } = dir;
    if (filePaths[0]) {
      const checkDirIsEmpty = fs
        .readdirSync(filePaths[0])
        .filter(
          (f) => !f.startsWith(".") /* If user has stuff related to git etc */,
        );

      if (checkDirIsEmpty.length !== 0) {
        e.reply("opened-directory", {
          canceled: false,
          filePaths: [],
          isNotEmpty: true,
        });
        return;
      }
    }

    e.reply("opened-directory", dir);
  });

  ipcMain.on("open-yml", async (e) => {
    const file = await dialog.showOpenDialog(mainWindow!, {
      properties: ["openFile", "createDirectory"],
      filters: [{ name: "Project File", extensions: ["yml", "yaml"] }],
    });
    e.reply("opened-yml", file);
  });

  // Initialising the yml file in the directory
  ipcMain.on("create-project-in-dir", (event, arg) => {
    try {
      const { path, projectTitle, projectDescription } = arg as {
        [k: string]: string;
      };
      const id = randomUUID();

      const config: Config | false = getConfigData();
      if (config) {
        const checkIfPathAlreadyExists = config.projectFiles.filter(
          (project) => project.path === path,
        );
        if (checkIfPathAlreadyExists.length > 0) {
          event.reply("create-error", "Project path already exists");
          return;
        }

        const data: ProjectFile = {
          id,
          path,
          title: projectTitle,
          description: projectDescription,
          dateModified: new Date(),
        };
        config.projectFiles.push(data);

        const updateConfig = saveConfigData(config);
        if (updateConfig) {
          // TODO: Create .yml file in the path dir
        } else {
          event.reply("error-create", "Error creating project");
        }
      }
    } catch (err) {}
  });
}
