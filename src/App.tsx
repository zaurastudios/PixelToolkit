import "./global.scss";

import { UnzipProgress } from "./components/unzip-progress";
import { Toaster } from "@/components/ui/sonner";
import { simpleToast } from "./lib/toast";

import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEffect } from "react";
import { Route, MemoryRouter as Router, Routes } from "react-router-dom";
import { Navigation } from "@/components/navigation";
import { Home } from "./screens/home";
import { ProjectPage } from "./screens/project";

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

      await simpleToast();
    };

    init();
  }, []);

  return (
    <>
      <Router>
        <Navigation />
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/project/:id" element={<ProjectPage />} />
        </Routes>
      </Router>
      <Toaster />
      <UnzipProgress />
    </>
  );
}

export default App;
