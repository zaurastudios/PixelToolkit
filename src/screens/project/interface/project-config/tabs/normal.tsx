import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { invoke } from "@tauri-apps/api/core";
import React, { useEffect } from "react";

export function Normal({ materialPath }: { materialPath: string }) {
  function submitHandler(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
  }

  useEffect(() => {
    async function init() {
      await invoke("select_texture_file", {
        materialPath,
        texture: "normal",
      });
    }

    init();
  }, [materialPath]);

  return (
    <form className="flex flex-col gap-2 font-mono" onSubmit={submitHandler}>
      <div className="grid grid-cols-2 items-center justify-center px-2">
        <Label className="text-center">Value</Label>
        <Input className="h-max p-0.5 text-center" name="value" type="number" />
      </div>
      <div className="grid grid-cols-2 items-center justify-center px-2">
        <Label className="text-center">Shift</Label>
        <Input className="h-max p-0.5 text-center" name="shift" type="number" />
      </div>
      <div className="grid grid-cols-2 items-center justify-center px-2">
        <Label className="text-center">Scale</Label>
        <Input className="h-max p-0.5 text-center" name="scale" type="number" />
      </div>
    </form>
  );
}
