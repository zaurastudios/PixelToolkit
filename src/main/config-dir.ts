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

export function getConfigData() {
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

export interface Config {
  projectFiles: ProjectFile[];
}

export interface ProjectFile {
  id: string;
  path: string;
  title: string;
  description: string;
  dateModified: Date;
}