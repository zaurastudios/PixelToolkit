import fs from "fs";
import path from "path";

const programFilesDir = "C:\\Program Files\\";
const homeDir = "$HOME/";

function getConfigDirPath() {
  let configDirPath;

  const platform = process.platform;
  switch (platform) {
    case "win32":
      configDirPath = path.join(programFilesDir, ".pixel-toolkit");
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

export function createConfigDIR() {
  const configDirPath = getConfigDirPath();

  // Create the config folder if it doesn't exist
  if (!fs.existsSync(configDirPath)) {
    fs.mkdirSync(configDirPath, { recursive: true });
  }

  return configDirPath;
}
