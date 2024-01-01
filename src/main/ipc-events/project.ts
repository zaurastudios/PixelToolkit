import { BrowserWindow, ipcMain, shell } from "electron";
import path from "path";
import fs from "fs";
import { getConfigData, getProjectData, removeProject } from "../config-dir";

export interface FileTreeProps {
  name: string;
  isMat?: boolean;
  children?: FileTreeProps[];
}

const unlistedDirs = [
  "blockstates",
  "models",
  "texts",
  "shaders",
  "lang",
  "font",
  "atlases",
  "colormap",
];
function treeBuilder(projectPath: string): FileTreeProps {
  const files = fs.readdirSync(projectPath);
  const node: FileTreeProps = {
    name: path.basename(projectPath),
    children: [],
  };

  files.forEach((file) => {
    const filePath = path.join(projectPath, file);
    const isDir = fs.statSync(filePath).isDirectory();

    if (isDir) {
      const children = treeBuilder(filePath);
      if (!unlistedDirs.includes(children.name.toLowerCase())) {
        node.children!.push(children);
      }

      // Check if dir contains a mat.yml file
      const isMatDir = fs
        .readdirSync(filePath)
        .filter((f) => f.match(/^mat.*\.(yml|yaml)$/i));
      if (isMatDir.length > 0) {
        children.isMat = true;
        children.children = [];
      }
    }
  });

  return node;
}

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
        shell.showItemInFolder(shellPath);
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
        console.error(err);
        event.reply("deleted-project", {
          redirect: "/",
          toast: "Unable to delete.",
          error: true,
        });
      }
    });

    ipcMain.on("get-project-file-tree", (event, projectPath) => {
      try {
        const fileTree = treeBuilder(projectPath);
        event.reply("project-file-tree", {
          redirect: false,
          fileTree,
        });
      } catch (err) {
        console.error(err);
        event.reply("project-file-tree", {
          redirect: true,
          message: "Failed to fetch file tree",
        });
      }
    });
  } catch (err) {
    console.error(err);
  }
}
