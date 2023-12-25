import { BrowserWindow, dialog, ipcMain } from "electron";

export default function CreateEventsHandlers(mainWindow: BrowserWindow) {
  // Open directory dialog
  ipcMain.on("open-directory", async (e) => {
    const dir = await dialog.showOpenDialog(mainWindow!, {
      properties: ["openDirectory"],
    });
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
