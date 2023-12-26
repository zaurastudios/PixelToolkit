import { MemoryRouter as Router, Routes, Route } from "react-router-dom";
import "./App.css";
import MenuBar from "./components/menu-bar";

import Home from "./Screens/Home";
import Project from "./Screens/Project";

export default function App() {
  return (
    <Router>
      <MenuBar />
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/:id" element={<Project />} />
      </Routes>
    </Router>
  );
}
