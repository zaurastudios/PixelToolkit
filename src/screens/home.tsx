import { Project } from "@/types/home";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export function Home() {
  const [projects, setProjects] = useState<Project[]>([]);

  useEffect(() => {
    async function getProjects() {
      const pr: Project[] = await invoke("get_projects");
      console.log(pr);
      setProjects(pr);
    }
    getProjects();
  }, []);

  return <></>;
}
