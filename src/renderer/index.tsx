import { createRoot } from "react-dom/client";
import { ThemeProvider } from "next-themes";
import { createPortal } from "react-dom";
import App from "./App";
import { Toaster } from "./components/ui/toaster";
import { Toaster as Sonner } from "./components/ui/sonner";

const container = document.getElementById("root") as HTMLElement;
const root = createRoot(container);
root.render(
  <ThemeProvider defaultTheme="dark" attribute="class">
    <App />
    {createPortal(<Toaster />, document.body)}
    {createPortal(<Sonner />, document.body)}
  </ThemeProvider>,
);

// calling IPC exposed from preload script
window.electron.ipcRenderer.once("ipc-example", (arg) => {
  // eslint-disable-next-line no-console
  console.log(arg);
});
window.electron.ipcRenderer.sendMessage("ipc-example", ["ping"]);
