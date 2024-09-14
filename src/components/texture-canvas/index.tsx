import { useKeyboardShortcut } from "@/lib/use-keyboard-shortcut";
import { OrbitControls, OrbitControlsProps, Stats } from "@react-three/drei";
import { Canvas, useLoader, useThree } from "@react-three/fiber";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useMemo, useRef, useState } from "react";
import {
  BufferAttribute,
  BufferGeometry,
  Color,
  NearestFilter,
  TextureLoader,
} from "three";

export function TextureCanvas() {
  const [selectedTexture, setSelectedTexture] = useState<string | null>();

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
        <meshBasicMaterial map={texture} toneMapped={false} />
      </mesh>
      {/* <GridOverlay */}
      {/*   width={planeWidth} */}
      {/*   height={planeHeight} */}
      {/*   resolutionX={texture.image.width} */}
      {/*   resolutionY={texture.image.height} */}
      {/* /> */}
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
    camera.position.set(0, 1.8369701987210297e-16, 0);
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
        maxDistance={3}
      />
    </>
  );
};

const CanvasWrapper = ({ children }: { children: React.ReactNode }) => {
  return (
    <Canvas>
      {children}
      <CameraController />
      <Stats />
    </Canvas>
  );
};

interface GridOverlayProps {
  width: number;
  height: number;
  resolutionX: number;
  resolutionY: number;
}

function GridOverlay({
  width,
  height,
  resolutionX,
  resolutionY,
}: GridOverlayProps) {
  const geometry = useMemo(() => {
    const geo = new BufferGeometry();
    const vertices = [];

    // Vertical lines
    for (let i = 0; i <= resolutionX; i++) {
      const x = (i / resolutionX - 0.5) * width;
      vertices.push(x, -height / 2, 0);
      vertices.push(x, height / 2, 0);
    }

    // Horizontal lines
    for (let j = 0; j <= resolutionY; j++) {
      const y = (j / resolutionY - 0.5) * height;
      vertices.push(-width / 2, y, 0);
      vertices.push(width / 2, y, 0);
    }

    geo.setAttribute(
      "position",
      new BufferAttribute(new Float32Array(vertices), 3),
    );
    return geo;
  }, [width, height, resolutionX, resolutionY]);

  return (
    <lineSegments geometry={geometry}>
      <lineBasicMaterial
        color={new Color(0.5, 0.5, 0.5)}
        transparent
        opacity={0.5}
      />
    </lineSegments>
  );
}
