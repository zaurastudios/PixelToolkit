import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { AddAdditionalType } from "@/types";
import { NormalMap } from "@/types/interface";
import { invoke } from "@tauri-apps/api/core";
import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";

export function Normal({
  materialPath,
  textureFileOption: textureOpt,
}: {
  materialPath: string;
  textureFileOption: any;
}) {
  const navigate = useNavigate();

  const defaultValues: NormalMap = {
    curveX: 0,
    curveY: 0,
    radiusSizeX: 0,
    radiusSizeY: 0,
    noiseAngle: 0,
    method: 0,
    strength: 0,
  };

  const [values, setValues] =
    useState<AddAdditionalType<NormalMap, string>>(defaultValues);

  async function init() {
    try {
      const res: string = await invoke("select_texture_file", {
        materialPath,
        texture: "normal",
      });
      const parsedRes: NormalMap | string = await JSON.parse(res);
      if (typeof parsedRes === "string") throw new Error(parsedRes);

      setValues(parsedRes);
    } catch (err) {
      console.error(err);
      toast(String(err));
      navigate("/");
    }
  }

  useEffect(() => {
    setValues(defaultValues);
    init();
  }, [materialPath, textureOpt]);

  return (
    <>
      <div className="overflow-hidden rounded-md bg-background pb-2">
        <h3 className="mb-2 border-b bg-foreground/10 px-2 pt-1 font-bold">
          Filtering
        </h3>
        <div className="flex flex-col gap-2 font-mono">
          <div className="grid grid-cols-2 items-center justify-center px-2">
            <Label className="text-center">Value</Label>
            <Input
              className="h-max p-0.5 text-center"
              name="value"
              type="number"
            />
          </div>
          <div className="grid grid-cols-2 items-center justify-center px-2">
            <Label className="text-center">Shift</Label>
            <Input
              className="h-max p-0.5 text-center"
              name="shift"
              type="number"
            />
          </div>
          <div className="grid grid-cols-2 items-center justify-center px-2">
            <Label className="text-center">Scale</Label>
            <Input
              className="h-max p-0.5 text-center"
              name="scale"
              type="number"
            />
          </div>
        </div>
      </div>
    </>
  );
}
