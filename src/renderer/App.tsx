import { MemoryRouter as Router, Routes, Route } from "react-router-dom";
import "./App.css";

import Home from "./Screens/Home";

export default function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Home />} />
      </Routes>
    </Router>
  );
}
