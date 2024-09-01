import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./global.scss";
import { getCurrentWindow } from "@tauri-apps/api/window";

function App() {
  useEffect(() => {
    const setTheme = async () => {
      const systemTheme = await getCurrentWindow().theme();
      const storedTheme = localStorage.getItem("theme") as
        | "light"
        | "dark"
        | null;
      const theme = storedTheme || systemTheme || "dark";

      document.documentElement.classList.remove("light", "dark");
      document.documentElement.classList.add(theme);

      if (!storedTheme) {
        localStorage.setItem("theme", theme);
      }
    };

    setTheme();
  }, []);
  return <></>;
}

export default App;
