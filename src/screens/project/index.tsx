import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { FileTree } from "@/types/project";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { FileTreeSidebar } from "./file-tree";
import { Interface } from "./interface";
import { toast } from "sonner";

export function ProjectPage() {
  const { id } = useParams();
  const navigate = useNavigate();

  const [data, setData] = useState<{
    fileTree: FileTree;
    redirect: boolean;
    projectPath: string;
  } | null>(null);

  async function getFileTree() {
    try {
      const res: string = await invoke("get_dirs", { projectId: id });
      const resData: {
        file_tree: FileTree;
        redirect: boolean;
        project_path: string;
      } = await JSON.parse(res);
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
          projectPath: resData.project_path,
        });
      }
    } catch (err) {
      navigate("/");
      console.error(`Failed to open project: ${err}\nID: ${id}`);
      toast("Failed to open project:" + String(err));
    }
  }

  useEffect(() => {
    getFileTree();
  }, []);

  if (!data) return null;

  return (
    <>
      <div className="h-8"></div>
      <div className="h-[calc(100vh-60px)]">
        <ResizablePanelGroup direction="horizontal" className="border-t">
          <ResizablePanel
            minSize={18}
            defaultSize={18}
            maxSize={22}
            className="!overflow-scroll"
          >
            <FileTreeSidebar data={data} getFileTree={getFileTree} />
          </ResizablePanel>
          <ResizableHandle />
          <ResizablePanel className="bg-foreground/5">
            <Interface projectPath={data?.projectPath} />
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>
    </>
  );
}
