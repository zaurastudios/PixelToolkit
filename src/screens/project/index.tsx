import { FileTree } from "@/types/project";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

export function ProjectPage() {
  const { id } = useParams();
  const [fileTree, setFileTree] = useState<FileTree | null>(null);

  useEffect(() => {
    async function init() {
      const data = await invoke("get_dirs", { projectId: id });
      console.log(data);
    }
    init();
  }, [id]);

  return <></>;
}
