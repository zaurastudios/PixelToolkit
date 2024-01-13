// eslint-disable-next-line
import { TextureFilesTypes } from "..";
import Color from "./color";
import Height from "./height";
import Rough from "./rough";

export default function DefaultTab({
  textureFileOption,
  texturePath,
}: {
  textureFileOption: TextureFilesTypes;
  texturePath: string;
}) {
  let Component = null;

  switch (textureFileOption) {
    case "General":
      Component = <div>hi</div>;
      break;

    case "Color":
      Component = <Color />;
      break;

    case "Height":
      Component = <Height />;
      break;

    case "Roughness":
      Component = <Rough texturePath={texturePath} />;
      break;

    default:
      Component = null;
      break;
  }

  return Component;
}
