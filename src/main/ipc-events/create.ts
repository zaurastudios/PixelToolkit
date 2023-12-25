import { BrowserWindow, dialog, ipcMain } from "electron";
import fs from "fs";

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
}
