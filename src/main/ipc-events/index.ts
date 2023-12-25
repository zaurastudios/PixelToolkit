import { BrowserWindow } from "electron";
import CreateEventsHandlers from "./create";

export default function AllEvents(mainWindow: BrowserWindow | null) {
  if (mainWindow) {
    CreateEventsHandlers(mainWindow);
  }
}
