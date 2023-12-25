import { createRoot } from "react-dom/client";
import { ThemeProvider } from "next-themes";
import App from "./App";
import { Toaster } from "./components/ui/toaster";
import { createPortal } from "react-dom";

const container = document.getElementById("root") as HTMLElement;
const root = createRoot(container);
root.render(
  <ThemeProvider defaultTheme="dark" attribute="class">
    <App />
    {createPortal(<Toaster />, document.body)}
  </ThemeProvider>,
);

// calling IPC exposed from preload script
window.electron.ipcRenderer.once("ipc-example", (arg) => {
  // eslint-disable-next-line no-console
  console.log(arg);
});
window.electron.ipcRenderer.sendMessage("ipc-example", ["ping"]);
