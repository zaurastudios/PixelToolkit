// eslint-disable-next-line
// import { TextureFilesTypes } from "..";

import { Color } from "./color";
import { Grayscale } from "./grayscale";
import { Height } from "./height";
import { Normal } from "./normal";

export function DefaultTab({
  textureFileOption,
  materialPath,
}: {
  textureFileOption: any;
  materialPath: string;
}) {
  let Component = null;

  switch (textureFileOption) {
    case "General":
      Component = <div>hi</div>;
      break;

    case "Color":
      Component = <Color materialPath={materialPath} />;
      break;

    case "Opacity":
      Component = (
        <Grayscale
          materialPath={materialPath}
          texture="opacity"
          textureFileOption={textureFileOption}
        />
      );
      break;

    case "Height":
      Component = <Height materialPath={materialPath} />;
      break;

    case "Normal":
      Component = <Normal materialPath={materialPath} />;
      break;

    case "Smoothness":
      Component = (
        <Grayscale
          materialPath={materialPath}
          texture="smooth"
          textureFileOption={textureFileOption}
        />
      );
      break;

    case "Roughness":
      Component = (
        <Grayscale
          materialPath={materialPath}
          texture="rough"
          textureFileOption={textureFileOption}
        />
      );
      break;

    case "Metal":
      Component = (
        <Grayscale
          materialPath={materialPath}
          texture="metal"
          textureFileOption={textureFileOption}
        />
      );
      break;

    case "F0":
      Component = (
        <Grayscale
          materialPath={materialPath}
          texture="f0"
          textureFileOption={textureFileOption}
        />
      );
      break;

    case "Porosity":
      Component = (
        <Grayscale
          materialPath={materialPath}
          texture="porosity"
          textureFileOption={textureFileOption}
        />
      );
      break;

    case "SSS":
      Component = (
        <Grayscale
          materialPath={materialPath}
          texture="sss"
          textureFileOption={textureFileOption}
        />
      );
      break;

    case "Emissive":
      Component = (
        <Grayscale
          materialPath={materialPath}
          texture="emissive"
          textureFileOption={textureFileOption}
        />
      );
      break;

    default:
      Component = null;
      break;
  }

  return Component;
}
