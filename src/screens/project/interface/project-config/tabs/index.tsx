// eslint-disable-next-line
// import { TextureFilesTypes } from "..";

import { Color } from "./color";
import { Greyscale } from "./greyscale";
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
      Component = <Greyscale materialPath={materialPath} texture="opacity" />;
      break;

    case "Height":
      Component = <Height materialPath={materialPath} />;
      break;

    case "Normal":
      Component = <Normal materialPath={materialPath} />;
      break;

    case "Smoothness":
      Component = <Greyscale materialPath={materialPath} texture="smooth" />;
      break;

    case "Roughness":
      Component = <Greyscale materialPath={materialPath} texture="rough" />;
      break;

    case "Porosity":
      Component = <Greyscale materialPath={materialPath} texture="porosity" />;
      break;

    case "SSS":
      Component = <Greyscale materialPath={materialPath} texture="sss" />;
      break;

    case "Emissive":
      Component = <Greyscale materialPath={materialPath} texture="emissive" />;
      break;

    default:
      Component = null;
      break;
  }

  return Component;
}
