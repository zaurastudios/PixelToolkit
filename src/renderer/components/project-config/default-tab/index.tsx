// eslint-disable-next-line
import { TextureFilesTypes } from "..";
import Color from "./color";
import Height from "./height";
import Rough from "./rough";

export default function DefaultTab({
  textureFileOption,
}: {
  textureFileOption: TextureFilesTypes;
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
      Component = <Rough />;
      break;

    default:
      Component = null;
      break;
  }

  return Component;
}
