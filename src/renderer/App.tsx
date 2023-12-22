import { MemoryRouter as Router, Routes, Route } from "react-router-dom";
import "./App.css";
import MenuBar from "./components/menu-bar";

import Home from "./Screens/Home";
import Test from "./Screens/test";

export default function App() {
  return (
    <Router>
      <MenuBar />
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/test" element={<Test />} />
      </Routes>
    </Router>
  );
}
