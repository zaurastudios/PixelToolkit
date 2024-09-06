import { FileTree } from "@/types/project";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { toast } from "sonner";

export function ProjectPage() {
  const navigate = useNavigate();
  const { id } = useParams();
  const [data, setData] = useState<{
    fileTree: FileTree;
    redirect: boolean;
  } | null>(null);
  const fileTree = data?.fileTree;
  console.log(fileTree);

  useEffect(() => {
    async function init() {
      try {
        const res: string = await invoke("get_dirs", { projectId: id });
        const resData: { file_tree: FileTree; redirect: boolean } =
          JSON.parse(res);
        if (typeof resData === "string") {
          navigate("/");
          throw new Error("404");
        }

        if (resData.redirect) {
          navigate("/");
        } else {
          setData({
            fileTree: resData.file_tree,
            redirect: resData.redirect,
          });
        }
      } catch (err) {
        navigate("/");
        console.error("Failed to open project:", String(err));
        toast("Failed to open project:" + String(err));
      }
    }
    init();
  }, [id]);

  return <></>;
}
