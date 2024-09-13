// eslint-disable-next-line
// import { TextureFilesTypes } from "..";

import { Color } from "./color";
import { Height } from "./height";
import { Normal } from "./normal";
import { Rough } from "./rough";
import { Smooth } from "./smooth";

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

    case "Height":
      Component = <Height materialPath={materialPath} />;
      break;

    case "Normal":
      Component = <Normal materialPath={materialPath} />;
      break;

    case "Smoothness":
      Component = <Smooth materialPath={materialPath} />;
      break;
    case "Roughness":
      Component = <Rough materialPath={materialPath} />;
      break;

    default:
      Component = null;
      break;
  }

  return Component;
}
