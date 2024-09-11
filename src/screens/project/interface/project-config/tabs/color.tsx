import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";

export function Color({ materialPath }: { materialPath: string }) {
  function submitHandler(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
  }

  useEffect(() => {
    async function init() {
      const res = await invoke("select_texture_file", {
        materialPath,
        texture: "color",
      });
      console.log(res);
    }

    init();
  }, [materialPath]);
  return <div>color</div>;
}
