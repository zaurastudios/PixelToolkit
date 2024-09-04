import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";

export function Home() {
  useEffect(() => {
    async function getLog() {
      const data = await invoke("get_projects");
      console.log(data);
    }

    getLog();
  });

  return <></>;
}
