import React, { Suspense, useEffect, useRef, useState } from "react";
import { Canvas, useLoader } from "@react-three/fiber";
import { TextureLoader } from "three";
import titleString from "@/utils/title";
import placeholder from "@/assets/placeholder.png";
import { TextureFilesTypes } from "../project-config";

type TextureFiles = {
  name: string;
  file: string;
};

function Mesh() {
  const meshRef = useRef(null);

  const [textureFile, setTextureFile] = useState<TextureFiles[]>([]);
  const [selectedTextureFile, setSelectedTextureFile] =
    useState<TextureFiles>();
  const [textureFileType, setTextureFileType] =
    useState<TextureFilesTypes>("Color");

  useEffect(() => {
    window.electron.ipcRenderer.sendMessage("select-texture-file", "color");
  }, []);

  window.electron.ipcRenderer.on("selected-texture", (selectedPath, textureF) =>
    setTextureFile(textureF as TextureFiles[]),
  );
  window.electron.ipcRenderer.on("selected-texture-file", (f) => {
    let selectedTexture: TextureFilesTypes;
    if (f) {
      selectedTexture = titleString(String(f)) as TextureFilesTypes;
    } else {
      selectedTexture = "Color";
    }
    setTextureFileType(selectedTexture);
  });
  useEffect(() => {
    if (textureFile.length) {
      setSelectedTextureFile(
        textureFile.filter((t) => t.name === textureFileType)[0],
      );
    }
  }, [textureFile, textureFileType]);

  const texture = useLoader(
    TextureLoader,
    selectedTextureFile?.file
      ? `atom://${selectedTextureFile?.file}`
      : placeholder,
  );
  console.log(selectedTextureFile);

  if (selectedTextureFile && selectedTextureFile.file) {
    return (
      <mesh ref={meshRef}>
        <planeGeometry args={[1, 1]} />
        <meshBasicMaterial map={texture} />
      </mesh>
    );
  }
}

export default function TwoDCanvas() {
  return (
    <Canvas>
      <Suspense fallback={null}>
        <Mesh />
      </Suspense>
    </Canvas>
  );
}
