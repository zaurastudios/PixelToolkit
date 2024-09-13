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

export interface DefaultsGreyscale {
  value: number | null;
  shift: number | null;
  scale: number | null;
}
