import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { isNumber } from "@/lib/utils";
import { AddAdditionalType } from "@/types";
import { DefaultsGrayscale } from "@/types/interface";
import { invoke } from "@tauri-apps/api/core";
import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";

export function Grayscale({
  materialPath,
  texture,
  textureFileOption: textureOpt,
}: {
  materialPath: string;
  texture: string;
  textureFileOption: any;
}) {
  const navigate = useNavigate();

  const defaultValues: DefaultsGrayscale = {
    value: 0.0,
    shift: 0.0,
    scale: 1.0,
  };

  const [values, setValues] =
    useState<AddAdditionalType<DefaultsGrayscale, string>>(defaultValues);
  const [disabled, setDisabled] = useState(false);

  function toString(o: Object) {
    return Object.fromEntries(
      Object.entries(o).map(([k, v]) => [k, String(v)]),
    );
  }

  async function updateDefaults() {
    try {
      const res = await invoke("update_defaults_grayscale", {
        materialPath,
        texture,
        ...toString(values),
      });
      const parsedRes: string | boolean = await JSON.parse(String(res));
      console.log(parsedRes);
      if (typeof parsedRes === "string") throw new Error(parsedRes);

      if (parsedRes) init();
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
    const name = e.getAttribute("name")! as keyof DefaultsGrayscale;
    const value = e.value;

    try {
      if (!isNumber(value)) throw new Error("Value is NaN");
      if (!value || value.length < 1) throw new Error("Value is null");
      let parsedValue = parseFloat(value);

      if (name === "value") {
        parsedValue = Math.max(0, Math.min(255, parsedValue));
      }

      setValues((prev) => {
        const newValues = { ...prev, [name]: parsedValue };
        updateDefaults();
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
  }

  async function init() {
    try {
      const res: string = await invoke("select_texture_file", {
        materialPath,
        texture,
      });
      const parsedRes: { use_og: boolean; values: DefaultsGrayscale } | string =
        await JSON.parse(res);
      if (typeof parsedRes === "string") throw new Error(parsedRes);

      setValues(parsedRes.values);
      setDisabled(parsedRes.use_og);
    } catch (err) {
      console.error(err);
      toast(String(err));
      navigate("/");
    }
  }

  useEffect(() => {
    init();
  }, [materialPath, textureOpt]);
  return (
    <form className="flex flex-col gap-2 font-mono">
      <div className="grid grid-cols-2 items-center justify-center px-2">
        <Label className="text-center">Value</Label>
        <Input
          className="h-max p-0.5 text-center"
          name="value"
          type="number"
          min={0}
          max={255}
          onKeyDown={onKeyDown}
          onBlur={(e) => onBlur(e.currentTarget)}
          value={values.value ?? undefined}
          onChange={(e) => {
            setValues((prev) => ({ ...prev, value: e.target.value }));
          }}
          disabled={disabled}
        />
      </div>
      <div className="grid grid-cols-2 items-center justify-center px-2">
        <Label className="text-center">Shift</Label>
        <Input
          className="h-max p-0.5 text-center"
          name="shift"
          type="number"
          step="0.001"
          onKeyDown={onKeyDown}
          onBlur={(e) => onBlur(e.currentTarget)}
          value={values.shift ?? undefined}
          onChange={(e) => {
            setValues((prev) => ({ ...prev, shift: e.target.value }));
          }}
          disabled={disabled}
        />
      </div>
      <div className="grid grid-cols-2 items-center justify-center px-2">
        <Label className="text-center">Scale</Label>
        <Input
          className="h-max p-0.5 text-center"
          name="scale"
          type="number"
          step="0.001"
          onKeyDown={onKeyDown}
          onBlur={(e) => onBlur(e.currentTarget)}
          value={values.scale ?? undefined}
          onChange={(e) => {
            setValues((prev) => ({
              ...prev,
              scale: e.target.value,
            }));
          }}
          disabled={disabled}
        />
      </div>
    </form>
  );
}
