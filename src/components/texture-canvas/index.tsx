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
  RepeatWrapping,
  TextureLoader,
} from "three";
import { Button } from "../ui/button";
import { Checkbox } from "../ui/checkbox";

export function TextureCanvas() {
  const [selectedTexture, setSelectedTexture] = useState<string | null>();

  useEffect(() => {
    const unlisten = listen<string>("selected-texture-file", (e) => {
      setSelectedTexture(`data:image/png;base64,${e.payload}`);
    });

    return () => {
      unlisten.then((unlistenFn) => unlistenFn());
    };
  }, []);

  if (!selectedTexture) return null;

  return <Renderer selectedTexture={selectedTexture} />;
}

const Renderer = ({ selectedTexture }: { selectedTexture: string }) => {
  const [isTiled, setIsTiled] = useState(false);
  const [grid, setGrid] = useState(false);

  const texture = useLoader(TextureLoader, selectedTexture);
  texture.minFilter = NearestFilter;
  texture.magFilter = NearestFilter;

  // if (isTiled) {
  //   texture.wrapS = RepeatWrapping;
  //   texture.wrapT = RepeatWrapping;
  //   texture.repeat.set(3, 3);
  // } else {
  //   texture.repeat.set(1, 1);
  // }
  useEffect(() => {
    if (isTiled) {
      texture.wrapS = RepeatWrapping;
      texture.wrapT = RepeatWrapping;
      texture.repeat.set(3, 3);
    } else {
      // @ts-ignore
      texture.wrapS = texture.wrapT = undefined;
      texture.repeat.set(1, 1);
    }
    texture.needsUpdate = true;
  }, [isTiled, texture]);

  let planeWidth = 1;
  let planeHeight = 1;

  if (selectedTexture) {
    const aspectRatio = texture.image.width / texture.image.height;

    planeWidth =
      aspectRatio > 1
        ? aspectRatio * (texture.image.width / 128)
        : texture.image.width / 128;
    planeHeight =
      aspectRatio > 1
        ? texture.image.height / 128
        : (1 / aspectRatio) * (texture.image.height / 128);
  }

  const toggleTiling = () => setIsTiled(!isTiled);
  const toggleGrid = () => setGrid(!grid);

  useKeyboardShortcut({
    key: "t",
    handler: toggleTiling,
  });
  useKeyboardShortcut({
    key: "g",
    handler: toggleGrid,
  });

  return (
    <>
      <div className="flex w-full gap-4 border-b bg-background/10 p-2.5">
        <div className="flex items-center space-x-2">
          <Checkbox
            id="tile"
            checked={isTiled}
            onCheckedChange={toggleTiling}
          />
          <label
            htmlFor="tile"
            className="text-sm font-bold leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
          >
            Tile
          </label>
        </div>
        <div className="flex items-center space-x-2">
          <Checkbox id="grid" checked={grid} onCheckedChange={toggleGrid} />
          <label
            htmlFor="grid"
            className="text-sm font-bold leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
          >
            Grid
          </label>
        </div>
      </div>
      <CanvasWrapper>
        <mesh>
          <planeGeometry
            args={[
              planeWidth * (isTiled ? 3 : 1),
              planeHeight * (isTiled ? 3 : 1),
            ]}
          />
          <meshBasicMaterial map={texture} toneMapped={false} />
        </mesh>

        {grid && (
          <GridOverlay
            width={planeWidth * (isTiled ? 3 : 1)}
            height={planeHeight * (isTiled ? 3 : 1)}
            resolutionX={texture.image.width * (isTiled ? 3 : 1)}
            resolutionY={texture.image.height * (isTiled ? 3 : 1)}
          />
        )}
      </CanvasWrapper>
    </>
  );
};

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
    camera.position.set(0, 6.8369701987210297e-16, 0);
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
        maxDistance={10}
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
