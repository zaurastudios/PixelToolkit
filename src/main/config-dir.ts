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
