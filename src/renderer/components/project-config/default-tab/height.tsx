import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import React from "react";

export default function Height() {
  function submitHandler(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
  }

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
