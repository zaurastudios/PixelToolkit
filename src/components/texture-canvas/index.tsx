import { TextureFilesTypes } from "@/types/interface";

import { Canvas, useLoader } from "@react-three/fiber";
import { Stats, OrbitControls } from "@react-three/drei";
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { TextureLoader } from "three";
import { NearestFilter } from "three";

const DEFAULT_TEXTURE_TYPE: TextureFilesTypes = "Color";

export function TextureCanvas() {
  const [selectedTexture, setSelectedTexture] = useState<string | null>();
  const [textureType, setTextureType] =
    useState<TextureFilesTypes>(DEFAULT_TEXTURE_TYPE);

  console.log(selectedTexture);

  useEffect(() => {
    const unlisten = listen<string>("selected-texture-file", (e) =>
      setSelectedTexture(e.payload),
    );
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  if (!selectedTexture) return null;

  const texture = useLoader(
    TextureLoader,
    `data:image/png;base64,${selectedTexture}`,
  );
  texture.minFilter = NearestFilter;
  texture.magFilter = NearestFilter;

  let planeWidth = 1;
  let planeHeight = 1;

  if (selectedTexture) {
    const aspectRatio = texture.image.width / texture.image.height;
    planeWidth = aspectRatio > 1 ? aspectRatio : 1;
    planeHeight = aspectRatio > 1 ? 1 : 1 / aspectRatio;
  }

  return (
    <CanvasWrapper>
      <mesh>
        <planeGeometry args={[planeWidth, planeHeight]} />
        <meshBasicMaterial map={texture} />
      </mesh>
    </CanvasWrapper>
  );
}

const CanvasWrapper = ({ children }: { children: React.ReactNode }) => {
  return (
    <Canvas>
      {children}
      <OrbitControls enableRotate={false} minDistance={0.1} />
      <Stats />
    </Canvas>
  );
};
