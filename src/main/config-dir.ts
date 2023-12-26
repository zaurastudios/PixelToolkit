import fs from "fs";
import path from "path";
import { homedir } from "os";

const homeDir = homedir();

function getConfigDirPath() {
  let configDirPath;

  const platform = process.platform;
  switch (platform) {
    case "win32":
      configDirPath = path.join(
        homeDir,
        "AppData",
        "Roaming",
        ".pixel-toolkit",
      );
      break;

    case "darwin":
    case "linux":
      configDirPath = path.join(homeDir, ".pixel-toolkit");
      break;

    default:
      throw new Error("Something went wrong");
  }

  return configDirPath;
}

export function createConfigDir() {
  try {
    const configDirPath = getConfigDirPath();
    const configFilePath = path.join(configDirPath, "config.json");

    if (!fs.existsSync(configDirPath)) {
      fs.mkdirSync(configDirPath, { recursive: true });
      if (!fs.existsSync(configFilePath)) {
        const defaultConfig = {
          projectFiles: [],
        };

        fs.writeFileSync(
          configFilePath,
          JSON.stringify(defaultConfig, null, 2),
        );
      }
    }

    return configFilePath;
  } catch (err) {
    console.error(err);
    return err;
  }
}

export function getConfigData(): Config | false {
  try {
    const configDirPath = getConfigDirPath();
    const configFilePath = path.join(configDirPath, "config.json");

    const rawConfig = fs.readFileSync(configFilePath, "utf8");

    const config = JSON.parse(rawConfig);
    return config;
  } catch (err) {
    console.log(err);
    return false;
  }
}

export function saveConfigData(config: Config) {
  try {
    const configDirPath = getConfigDirPath();
    const configFilePath = path.join(configDirPath, "config.json");

    fs.writeFileSync(configFilePath, JSON.stringify(config, null, 2));
    return true;
  } catch (err) {
    console.error(err);
    return false;
  }
}

export function getProjectData(id: string): ProjectFile | false {
  try {
    const config = getConfigData();
    if (config) {
      const { projectFiles } = config;
      const project = projectFiles.filter((file) => file.id === id);

      if (project.length > 0) {
        const updatedProject = project[0];
        updatedProject.dateModified = new Date();

        const newConfig = config;
        newConfig.projectFiles = newConfig.projectFiles.filter(
          (file) => file.id !== id,
        );
        newConfig.projectFiles.push(updatedProject);
        saveConfigData(newConfig);

        return project[0];
      }
    }

    throw new Error("No project found");
  } catch (err) {
    console.error(err);
    return false;
  }
}

export interface Config {
  projectFiles: ProjectFile[];
}

export interface ProjectFile {
  id: string;
  path: string;
  name: string;
  description: string;
  dateModified: Date;
}
