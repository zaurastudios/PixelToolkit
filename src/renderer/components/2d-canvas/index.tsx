import React, { Suspense, useEffect, useRef, useState } from "react";
import { Canvas, useLoader } from "@react-three/fiber";
import * as THREE from "three";
import { TextureLoader } from "three";
import titleString from "@/utils/title";
import placeholder from "@/assets/placeholder.png";
import { Stats, OrbitControls } from "@react-three/drei";
import { TextureFilesTypes } from "../project-config";

const DEFAULT_TEXTURE_TYPE: TextureFilesTypes = "Color";

type TextureFiles = {
  name: string;
  file: string;
};

function Mesh() {
  const meshRef = useRef(null);

  const [textureFiles, setTextureFiles] = useState<TextureFiles[]>([]);
  const [selectedTexture, setSelectedTexture] = useState<TextureFiles>();
  const [textureType, setTextureType] =
    useState<TextureFilesTypes>(DEFAULT_TEXTURE_TYPE);

  useEffect(() => {
    const handleSelectedTextureFile = (f: any) => {
      const selectedTextureH = ["color", "general"].includes(
        String(f).toLowerCase(),
      )
        ? DEFAULT_TEXTURE_TYPE
        : (titleString(String(f)) as TextureFilesTypes);
      setTextureType(selectedTextureH);
    };

    window.electron.ipcRenderer.sendMessage("select-texture-file", "color");

    window.electron.ipcRenderer.on("selected-texture", (_, textureF) => {
      setTextureFiles(textureF as TextureFiles[]);
    });

    window.electron.ipcRenderer.on(
      "selected-texture-file",
      handleSelectedTextureFile,
    );
  }, []);

  useEffect(() => {
    if (textureFiles.length) {
      setSelectedTexture(textureFiles.find((t) => t.name === textureType));
    }
  }, [textureFiles, textureType]);

  const texture = useLoader(
    TextureLoader,
    selectedTexture?.file ? `atom://${selectedTexture?.file}` : placeholder,
  );
  texture.minFilter = THREE.NearestFilter;
  texture.magFilter = THREE.NearestFilter;

  let planeWidth = 1;
  let planeHeight = 1;

  if (selectedTexture && selectedTexture.file) {
    const aspectRatio = texture.image.width / texture.image.height;
    planeWidth = aspectRatio > 1 ? aspectRatio : 1;
    planeHeight = aspectRatio > 1 ? 1 : 1 / aspectRatio;
  }

  if (selectedTexture && selectedTexture.file) {
    return (
      <mesh ref={meshRef}>
        <planeGeometry args={[planeWidth, planeHeight]} />
        <meshBasicMaterial map={texture} />
      </mesh>
    );
  }

  return null;
}

export default function TwoDCanvas() {
  return (
    <Canvas>
      <Suspense fallback={null}>
        <Mesh />
        <OrbitControls enableRotate={false} minDistance={0.1} maxDistance={5} />
        <Stats />
      </Suspense>
    </Canvas>
  );
}
