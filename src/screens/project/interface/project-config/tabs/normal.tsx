import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { isNumber, toString } from "@/lib/utils";
import { AddAdditionalType } from "@/types";
import { kernelSizes, NormalMap } from "@/types/interface";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
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
    radiusSizeX: 0.5,
    radiusSizeY: 0.5,
    noiseAngle: 0,
    method: 0,
    strength: 1,
  };

  const [values, setValues] =
    useState<AddAdditionalType<NormalMap, string>>(defaultValues);

  async function updateDefaults() {
    try {
      console.log(toString(values));
      const res = await invoke("update_normals", {
        materialPath,
        ...toString(values),
      });
      const parsedRes: string | boolean = await JSON.parse(String(res));
      if (typeof parsedRes === "string") throw new Error(parsedRes);

      if (parsedRes) {
        init();
      }
    } catch (err) {
      console.error(err);
      toast(String(err));
      navigate("/");
    }
  }

  function onKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    if (e.key === "Escape" || e.key === "Enter") onBlur(e.currentTarget);
  }

  function onBlur(e: EventTarget & HTMLInputElement) {
    const name = e.getAttribute("name")! as keyof NormalMap;
    const value = e.value;

    try {
      if (!isNumber(value)) throw new Error("Value is NaN");
      if (!value || value.length < 1) throw new Error("Value is null");
      let parsedValue = parseFloat(value);

      setValues((prev) => {
        const newValues = { ...prev, [name]: parsedValue };
        return newValues;
      });
    } catch (err) {
      console.log(err);
      const defaultValue = defaultValues[name];
      setValues((prev) => ({
        ...prev,
        [name]: defaultValue,
      }));
    }
    updateDefaults();
  }

  async function init() {
    try {
      const res: string = await invoke("select_texture_file", {
        materialPath,
        texture: "normal",
      });
      const parsedRes: NormalMapRes | string = await JSON.parse(res);
      if (typeof parsedRes === "string") throw new Error(parsedRes);

      setValues({
        curveX: parsedRes.curve_x,
        curveY: parsedRes.curve_y,
        radiusSizeX: parsedRes.radius_size_x,
        radiusSizeY: parsedRes.radius_size_y,
        noiseAngle: parsedRes.noise_angle,
        // @ts-ignore
        method: parsedRes.method,
        strength: parsedRes.strength,
      });
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
      <div className="overflow-hidden rounded-md bg-background pb-2 shadow">
        <h3 className="mb-2 border-b bg-foreground/10 px-2 pt-1 font-semibold">
          Filtering
        </h3>
        <div className="flex flex-col gap-2 font-mono">
          <div className="grid grid-cols-2 items-center justify-center px-2">
            <Label className="text-center">Curve X</Label>
            <Input
              className="h-max p-0.5 text-center"
              name="curve-X"
              type="number"
              onKeyDown={onKeyDown}
              onBlur={(e) => onBlur(e.currentTarget)}
              value={values.curveX ?? undefined}
              onChange={(e) => {
                setValues((prev) => ({
                  ...prev,
                  curveX: e.target.value,
                }));
              }}
            />
          </div>
          <div className="grid grid-cols-2 items-center justify-center px-2">
            <Label className="text-center">Curve Y</Label>
            <Input
              className="h-max p-0.5 text-center"
              name="curveY"
              type="number"
              onKeyDown={onKeyDown}
              onBlur={(e) => onBlur(e.currentTarget)}
              value={values.curveY ?? undefined}
              onChange={(e) => {
                setValues((prev) => ({
                  ...prev,
                  curveY: e.target.value,
                }));
              }}
            />
          </div>
          <div aria-hidden className="h-2" />
          <div className="grid grid-cols-2 items-center justify-center px-2">
            <Label className="text-center">Radius Size X</Label>
            <Input
              className="h-max p-0.5 text-center"
              name="radiusSizeX"
              type="number"
              onKeyDown={onKeyDown}
              onBlur={(e) => onBlur(e.currentTarget)}
              value={values.radiusSizeX ?? undefined}
              onChange={(e) => {
                setValues((prev) => ({
                  ...prev,
                  radiusSizeX: e.target.value,
                }));
              }}
            />
          </div>
          <div className="grid grid-cols-2 items-center justify-center px-2">
            <Label className="text-center">Radius Size Y</Label>
            <Input
              className="h-max p-0.5 text-center"
              name="radiusSizeY"
              type="number"
              onKeyDown={onKeyDown}
              onBlur={(e) => onBlur(e.currentTarget)}
              value={values.radiusSizeY ?? undefined}
              onChange={(e) => {
                setValues((prev) => ({
                  ...prev,
                  radiusSizeY: e.target.value,
                }));
              }}
            />
          </div>
          <div aria-hidden className="h-2" />
          <div className="grid grid-cols-2 items-center justify-center px-2">
            <Label className="text-center">Noise Angle</Label>
            <Input
              className="h-max p-0.5 text-center"
              name="noiseAngle"
              type="number"
              value={values.noiseAngle ?? undefined}
              onChange={(e) => {
                setValues((prev) => ({
                  ...prev,
                  noiseAngle: e.target.value,
                }));
              }}
            />
          </div>
        </div>
      </div>

      <div className="overflow-hidden rounded-md bg-background pb-2 shadow">
        <h3 className="mb-2 border-b bg-foreground/10 px-2 pt-1 font-semibold">
          Generate from height
        </h3>
        <div className="flex flex-col gap-2 font-mono">
          <div className="grid grid-cols-2 items-center justify-center px-2">
            <Label className="text-center">Method</Label>
            <Select
              name="method"
              value={String(values.method)}
              onValueChange={(e) => {
                console.log(e);
                setValues((prev) => ({
                  ...prev,
                  method: e,
                }));
              }}
              onOpenChange={(e) => {
                if (!e) updateDefaults();
              }}
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {Object.entries(kernelSizes).map(([key, value]) => (
                  <SelectItem key={key} value={key}>
                    {value}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <div className="grid grid-cols-2 items-center justify-center px-2">
            <Label className="text-center">Strength</Label>
            <Input
              className="h-max p-0.5 text-center"
              name="strength"
              type="number"
              onKeyDown={onKeyDown}
              onBlur={(e) => onBlur(e.currentTarget)}
              value={values.strength ?? undefined}
              onChange={(e) => {
                setValues((prev) => ({
                  ...prev,
                  strength: e.target.value,
                }));
              }}
            />
          </div>
        </div>
      </div>
    </>
  );
}

export interface NormalMapRes {
  // Filtering
  curve_x: number | null;
  curve_y: number | null;

  radius_size_x: number | null;
  radius_size_y: number | null;

  noise_angle: number | null;

  // Generate from height
  method: 0 | 1 | 2 | 3 | 4;
  strength: number | null;
}
