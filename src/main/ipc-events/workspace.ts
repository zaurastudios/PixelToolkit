import { BrowserWindow, ipcMain } from "electron";
import fs from "fs";
import YAML from "yaml";
import titleString from "../../renderer/utils/title";

const textureFiles = [
  {
    name: "color",
    match: /.*(albedo|color).*\.png$/i,
  },
  {
    name: "opacity",
    match: /.*opacity.*\.png$/i,
  },
  {
    name: "height",
    match: /.*height.*\.png$/i,
  },
  {
    name: "normal",
    match: /.*normal.*\.png$/i,
  },
  {
    name: "occlusion",
    match: /.*(occlusion|ao).*\.png$/i,
  },
  {
    name: "smooth",
    match: /.*smooth.*\.png$/i,
  },
  {
    name: "rough",
    match: /.*rough.*\.png$/i,
  },
  {
    name: "metal",
    match: /.*metal.*\.png$/i,
  },
  {
    name: "hcm",
    match: /.*hcm.*\.png$/i,
  },
  {
    name: "f0",
    match: /.*f0.*\.png$/i,
  },
  {
    name: "porosity",
    match: /.*porosity.*\.png$/i,
  },
  {
    name: "sss",
    match: /.*sss.*\.png$/i,
  },
  {
    name: "emissive",
    match: /.*emissive.*\.png$/i,
  },
];

export default function WorkspaceEventsHandler(mainWindow: BrowserWindow) {
  try {
    ipcMain.on("select-texture", (event, selectedPath) => {
      const files = fs.readdirSync(selectedPath);
      const pathToTextureFiles = textureFiles.map((texture) => {
        const textureFile = files.filter((file) => file.match(texture.match));
        return {
          name: titleString(texture.name),
          file: textureFile.length ? `${selectedPath}/${textureFile[0]}` : null,
        };
      });

      event.reply("selected-texture", selectedPath, pathToTextureFiles);
    });

    ipcMain.on("select-texture-file", (event, f) => {
      event.reply("selected-texture-file", (f as string).toLowerCase());
    });

    ipcMain.on("get-texture-values", (event, projectPath, textureOpt) => {
      const matYmlPath = `${projectPath}/mat.yml`;
      const ymlData: { [k: string]: unknown } = YAML.parse(
        fs.readFileSync(matYmlPath, "utf8"),
      );
      if (ymlData[textureOpt]) {
        event.reply("texture-values", ymlData[textureOpt]);
      } else {
        event.reply("texture-values", null);
      }
    });
  } catch (err) {
    console.error(err);
  }
}
