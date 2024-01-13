import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import React, { useEffect, useState } from "react";
import { Roughness } from "../types";

export default function Rough({ texturePath }: { texturePath: string }) {
  const [textureValues, setTextureValues] = useState<Roughness | null>();

  function submitHandler(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
  }

  useEffect(() => {
    window.electron.ipcRenderer.sendMessage(
      "get-texture-values",
      texturePath,
      "rough",
    );

    window.electron.ipcRenderer.on("texture-values", (data) => {
      if (data) {
        setTextureValues(data as Roughness);
      }
    });
  }, [texturePath]);

  return (
    <form className="flex flex-col gap-2 font-mono" onSubmit={submitHandler}>
      <div className="grid grid-cols-2 items-center justify-center px-2">
        <Label className="text-center">Value</Label>
        <Input
          className="h-max p-0.5 text-center"
          name="value"
          type="number"
          step="0.001"
          defaultValue={textureValues?.value ?? 0}
        />
      </div>
      <div className="grid grid-cols-2 items-center justify-center px-2">
        <Label className="text-center">Shift</Label>
        <Input
          className="h-max p-0.5 text-center"
          name="shift"
          type="number"
          step="0.001"
          defaultValue={textureValues?.shift ?? 0}
        />
      </div>
      <div className="grid grid-cols-2 items-center justify-center px-2">
        <Label className="text-center">Scale</Label>
        <Input
          className="h-max p-0.5 text-center"
          name="scale"
          type="number"
          step="0.001"
          defaultValue={textureValues?.scale ?? 0}
        />
      </div>
    </form>
  );
}
