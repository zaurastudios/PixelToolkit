import { MemoryRouter as Router, Routes, Route } from "react-router-dom";
import "./App.css";
import { AnimatePresence, motion } from "framer-motion";
import * as Card from "@/components/ui/card";
import { createPortal } from "react-dom";
import MenuBar from "./components/menu-bar";

import Home from "./Screens/Home";
import Project from "./Screens/Project";

import useCustomContextFileTree from "./lib/custom-context-menu";
import { Button } from "./components/ui/button";

export default function App() {
  return (
    <Router>
      <MenuBar />
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/:id" element={<Project />} />
      </Routes>
      {/* eslint-disable-next-line */}
      <ContextMenu />
    </Router>
  );
}

const ContextMenu = () => {
  const { points, clicked, path } = useCustomContextFileTree();
  const MotionCardComponent = motion(Card.Card);

  return createPortal(
    <AnimatePresence>
      {clicked && path && (
        <MotionCardComponent
          initial={{ opacity: 0, translateY: 20 }}
          animate={{ opacity: 1, translateY: 0 }}
          exit={{ opacity: 0, translateY: 20 }}
          transition={{ duration: 0.1 }}
          className="fixed bg-background p-1 shadow"
          style={{
            top: points.y,
            left: points.x,
          }}
        >
          <Button
            variant="ghost"
            onClick={() =>
              window.electron.ipcRenderer.sendMessage("open-in-folder", path)
            }
          >
            Open in Folder
          </Button>
        </MotionCardComponent>
      )}
    </AnimatePresence>,
    document.body,
  );
};
