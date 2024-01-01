import React, { useState, useEffect } from "react";
import { useLocation } from "react-router-dom";

const useCustomContextFileTree = () => {
  const location = useLocation();
  const [clicked, setClicked] = useState(false);
  const [points, setPoints] = useState({
    x: 0,
    y: 0,
  });
  const [path, setPath] = useState<string>();

  const handleContextMenu = (e: React.MouseEvent) => {
    const targetElement = e.target as HTMLElement;
    const isFileFolder =
      targetElement.getAttribute("data-label") === "filefolder" ||
      (targetElement.parentElement &&
        targetElement.parentElement.getAttribute("data-label") ===
          "filefolder");

    if (isFileFolder) {
      e.preventDefault();
      setPoints({ x: e.clientX, y: e.clientY });
      setClicked(true);

      try {
        let targetPath;
        if (targetElement.getAttribute("data-label") === "filefolder") {
          targetPath = targetElement.getAttribute("data-path");
        } else {
          targetPath =
            targetElement.parentElement &&
            targetElement.parentElement.getAttribute("data-path");
        }

        if (targetPath) {
          setPath(targetPath);
        }
      } catch (err) {
        console.error(err);
      }
    }
  };

  const handleClick = () => {
    setClicked(false);
    setPath("");
  };

  useEffect(() => {
    // @ts-ignore
    document.addEventListener("contextmenu", handleContextMenu);
    document.addEventListener("click", handleClick);

    return () => {
      document.removeEventListener("click", handleClick);
      // @ts-ignore
      document.removeEventListener("contextmenu", handleContextMenu);
    };
  }, []);
  useEffect(() => {
    if (clicked) {
      document.addEventListener("wheel", handleClick);
    }
    return () => {
      if (clicked) {
        document.removeEventListener("wheel", () => handleClick);
      }
    };
  }, [clicked]);
  useEffect(() => handleClick(), [location]);

  return {
    clicked,
    setClicked,
    points,
    setPoints,
    path,
  };
};
export default useCustomContextFileTree;
