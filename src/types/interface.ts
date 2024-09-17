export const textureFilesOptions = [
  "General",
  "Color",
  "Opacity",
  "Height",
  "Normal",
  "Occlusion",
  "Smoothness",
  "Roughness",
  "Metal",
  "HCM",
  "F0",
  "Porosity",
  "SSS",
  "Emissive",
] as const;
export type TextureFilesTypes = (typeof textureFilesOptions)[number];

export interface DefaultsGrayscale {
  value: number | null;
  shift: number | null;
  scale: number | null;
}

export const kernelSizes = {
  0: "Sobel 3x3",
  1: "Sobel 5x5",
  2: "Sobel 9x9",
  3: "Sobel-Low",
  4: "Sobel-High",
};
export interface NormalMap {
  // Filtering
  curveX: number | null;
  curveY: number | null;

  radiusSizeX: number | null;
  radiusSizeY: number | null;

  noiseAngle: number | null;

  // Generate from height
  method: 0 | 1 | 2 | 3 | 4;
  strength: number | null;
}
