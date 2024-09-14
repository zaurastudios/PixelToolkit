import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { isNumber } from "@/lib/utils";
import { AddAdditionalType } from "@/types";
import { DefaultsGrayscale } from "@/types/interface";
import { invoke } from "@tauri-apps/api/core";
import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useDebounce } from "react-use";
import { toast } from "sonner";

export function Grayscale({
  materialPath,
  texture,
}: {
  materialPath: string;
  texture: string;
}) {
  const naviagte = useNavigate();

  const defaultValues: DefaultsGrayscale = {
    value: 0.0,
    shift: 0.0,
    scale: 1.0,
  };

  const [isInit, setIsInit] = useState(false);
  const [values, setValues] =
    useState<AddAdditionalType<DefaultsGrayscale, string>>(defaultValues);
  const [disabled, setDisabled] = useState(false);

  function toString(o: Object) {
    Object.keys(o).forEach((k) => {
      // @ts-ignore
      o[k] = "" + o[k];
    });

    return o;
  }

  useDebounce(
    async () => {
      if (isInit) {
        try {
          const res = await invoke("update_defaults_grayscale", {
            materialPath,
            texture,
            ...toString(values),
          });
          const parsedRes: string | boolean = await JSON.parse(String(res));
          if (typeof parsedRes === "string") throw new Error(parsedRes);

          if (parsedRes) init();
        } catch (err) {
          console.error(err);
          toast(String(err));
          naviagte("/");
        }
      }
    },
    500,
    [values],
  );

  function onKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    if (e.key === "Escape" || e.key === "Enter") {
      const name = e.currentTarget.getAttribute(
        "name",
      )! as keyof DefaultsGrayscale;
      const value = e.currentTarget.value;

      try {
        if (!isNumber(value)) throw new Error("Value is NaN");
        if (!value || value.length < 1) throw new Error("Value is null");
        let parsedValue = parseFloat(value);

        if (name === "value") {
          parsedValue = Math.max(0, Math.min(255, parsedValue));
        }

        setValues((prev) => ({ ...prev, [name]: parsedValue }));
        e.currentTarget.value = String(parsedValue);
      } catch (err) {
        console.log(err);
        const defaultValue = defaultValues[name];
        setValues((prev) => ({
          ...prev,
          [name]: defaultValue,
        }));
        e.currentTarget.value = String(defaultValue);
      }
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
      setIsInit(true);
    } catch (err) {
      console.error(err);
      toast(String(err));
      naviagte("/");
    }
  }

  useEffect(() => {
    setIsInit(false);
    init();
  }, [materialPath]);

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
