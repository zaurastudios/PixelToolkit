import "./global.scss";

import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEffect } from "react";
import { Route, MemoryRouter as Router, Routes } from "react-router-dom";
import { Navigation } from "@/components/navigation";
import { Home } from "./screens/home";

function App() {
  useEffect(() => {
    const init = async () => {
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

    init();
  }, []);

  return (
    <>
      <Router>
        <Navigation />
        <Routes>
          <Route path="/" element={<Home />} />
          {/* <Route path="/:id" element={<Project />} /> */}
        </Routes>
      </Router>
    </>
  );
}

export default App;
