import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { invoke } from "@tauri-apps/api/core";
import React, { useEffect, useState } from "react";
// import { Roughness } from "../types";

export function Rough({ materialPath }: { materialPath: string }) {
  function submitHandler(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
  }

  console.log(materialPath);

  useEffect(() => {
    async function init() {
      const res = await invoke("select_texture_file", {
        materialPath,
        texture: "rough",
      });
      console.log(res);
    }

    init();
  }, [materialPath]);

  return (
    <form className="flex flex-col gap-2 font-mono" onSubmit={submitHandler}>
      <div className="grid grid-cols-2 items-center justify-center px-2">
        <Label className="text-center">Value</Label>
        <Input
          className="h-max p-0.5 text-center"
          name="value"
          type="number"
          step="0.001"
          // defaultValue={textureValues?.value ?? 0}
        />
      </div>
      <div className="grid grid-cols-2 items-center justify-center px-2">
        <Label className="text-center">Shift</Label>
        <Input
          className="h-max p-0.5 text-center"
          name="shift"
          type="number"
          step="0.001"
          // defaultValue={textureValues?.shift ?? 0}
        />
      </div>
      <div className="grid grid-cols-2 items-center justify-center px-2">
        <Label className="text-center">Scale</Label>
        <Input
          className="h-max p-0.5 text-center"
          name="scale"
          type="number"
          step="0.001"
          // defaultValue={textureValues?.scale ?? 0}
        />
      </div>
    </form>
  );
}
