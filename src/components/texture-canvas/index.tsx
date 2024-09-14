import { TextureFilesTypes } from "@/types/interface";

import { useKeyboardShortcut } from "@/lib/use-keyboard-shortcut";
import { OrbitControls, OrbitControlsProps } from "@react-three/drei";
import { Canvas, useLoader, useThree } from "@react-three/fiber";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useRef, useState } from "react";
import { NearestFilter, TextureLoader } from "three";

const DEFAULT_TEXTURE_TYPE: TextureFilesTypes = "Color";

export function TextureCanvas() {
  const [selectedTexture, setSelectedTexture] = useState<string | null>();
  const [textureType, setTextureType] =
    useState<TextureFilesTypes>(DEFAULT_TEXTURE_TYPE);

  useEffect(() => {
    const unlisten = listen<string>("selected-texture-file", (e) =>
      setSelectedTexture(e.payload),
    );
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  if (!selectedTexture) return null;

  console.log(`data:image/png;base64,${selectedTexture}`);
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
        <meshBasicMaterial map={texture} toneMapped={false} />
      </mesh>
    </CanvasWrapper>
  );
}

const CameraController = () => {
  const { camera, gl } = useThree();
  const controlsRef = useRef<OrbitControlsProps>(null);

  useEffect(() => {
    if (controlsRef.current) {
      const controls = controlsRef.current;
      controls.enableRotate = false;
      controls.minDistance = 0.1;
    }
  }, []);

  const resetCamera = () => {
    camera.position.set(0, 0, 1);
    camera.lookAt(0, 0, 0);
    // @ts-ignore
    controlsRef.current.reset();
  };

  useKeyboardShortcut({
    key: "F",
    handler: resetCamera,
  });

  return (
    <>
      <OrbitControls
        // @ts-ignore
        ref={controlsRef}
        args={[camera, gl.domElement]}
        enableRotate={false}
        minDistance={0.1}
      />
    </>
  );
};

const CanvasWrapper = ({ children }: { children: React.ReactNode }) => {
  return (
    <Canvas>
      {children}
      <CameraController />
    </Canvas>
  );
};
