import { randomUUID } from "crypto";
import { BrowserWindow, dialog, ipcMain } from "electron";
import fs from "fs";
import path from "path";
import YAML from "yaml";
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
      const {
        path: projectPath,
        projectTitle,
        projectDescription,
      } = arg as {
        [k: string]: string;
      };
      const id = randomUUID();

      const config: Config | false = getConfigData();
      if (config) {
        const checkIfPathAlreadyExists = config.projectFiles.filter(
          (project) => project.path === projectPath,
        );
        if (checkIfPathAlreadyExists.length > 0) {
          event.reply("error-create", "Project path already exists");
          return;
        }

        const data: ProjectFile = {
          id,
          path: projectPath,
          name: projectTitle,
          description: projectDescription,
          dateModified: new Date(),
        };
        config.projectFiles.push(data);

        const updateConfig = saveConfigData(config);
        if (updateConfig) {
          const ymlPath = path.join(projectPath, "project.yml");

          const doc: { [k: string]: any } = {};
          doc.name = projectTitle;
          doc.description = projectDescription;
          fs.writeFileSync(ymlPath, YAML.stringify(doc), "utf8");

          // Create the default blocks folder
          fs.mkdirSync(
            path.join(projectPath, "/assets/minecraft/textures/block"),
            { recursive: true },
          );

          event.reply("created", { id });
        } else {
          event.reply("error-create", "Error creating project");
        }
      }
    } catch (err) {
      console.error(err);
      event.reply("error-create", "Error creating project");
    }
  });

  // Select existing project
  ipcMain.on("selected-path-in-config", (event, selectedPath: string) => {
    try {
      const config = getConfigData();

      // Making sure the selected file is a project.yml file
      const checkFile = selectedPath.includes("project.yml");
      if (!checkFile) {
        event.reply("selected-path-is-in-config", {
          isPresent: true,
          message: "Please select a valid <code>project.yml</code> file.",
        });
        return;
      }

      if (config) {
        const parentPath = selectedPath.replace("/project.yml", "");
        const isThere = config.projectFiles.some(
          (project) => project.path === parentPath,
        );

        if (isThere) {
          event.reply("selected-path-is-in-config", {
            isPresent: true,
            message:
              "The path you've selected already exists, please choose a different path.",
          });
        } else {
          event.reply("selected-path-is-in-config", {
            isPresent: false,
          });
        }
      }
    } catch (err) {
      console.error(err);
      event.reply("selected-path-is-in-config", {
        isPresent: true,
        message: "Something happened! Please try again.",
      });
    }
  });

  ipcMain.on("get-yml-data", (event, ymlPath) => {
    try {
      const ymlData: { name: string; description: string } = YAML.parse(
        fs.readFileSync(ymlPath, "utf8"),
      );

      if (ymlData.name) {
        event.reply("yml-data", ymlData);
      } else {
        event.reply("yml-data", false);
      }
    } catch (err) {
      console.error(err);
      event.reply("yml-data", false);
    }
  });

  ipcMain.on("add-existing-project", (event, arg) => {
    try {
      const {
        ymlPath: projectPath,
        projectTitle,
        projectDescription,
      } = arg as {
        [k: string]: string;
      };
      const id = randomUUID();
      const parentPath = projectPath.replace("/project.yml", "");

      const config: Config | false = getConfigData();
      if (config) {
        const checkIfPathAlreadyExists = config.projectFiles.filter(
          (project) => project.path === parentPath,
        );
        if (checkIfPathAlreadyExists.length > 0) {
          event.reply("error-create", "Project path already exists");
          return;
        }

        const data: ProjectFile = {
          id,
          path: parentPath,
          name: projectTitle,
          description: projectDescription,
          dateModified: new Date(),
        };
        config.projectFiles.push(data);

        const updateConfig = saveConfigData(config);
        if (updateConfig) {
          const ymlPath = path.join(parentPath, "project.yml");

          const doc: { [k: string]: any } = {};
          doc.name = projectTitle;
          doc.description = projectDescription;
          fs.writeFileSync(ymlPath, YAML.stringify(doc), "utf8");

          event.reply("created", { id });
        } else {
          event.reply("error-create", "Error adding project");
        }
      }
    } catch (err) {
      console.error(err);
      event.reply("error-create", "Error adding project");
    }
  });
}
