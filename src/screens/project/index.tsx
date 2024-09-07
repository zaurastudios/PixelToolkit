import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { FileTree } from "@/types/project";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { toast } from "sonner";
import { FileTreeSidebar } from "./file-tree";

export function ProjectPage() {
  const navigate = useNavigate();
  const { id } = useParams();
  const [data, setData] = useState<{
    fileTree: FileTree;
    redirect: boolean;
    projectPath: string;
  } | null>(null);
  const fileTree = data?.fileTree;

  useEffect(() => {
    async function init() {
      try {
        const res: string = await invoke("get_dirs", { projectId: id });
        const resData: {
          file_tree: FileTree;
          redirect: boolean;
          project_path: string;
        } = JSON.parse(res);
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
        console.error("Failed to open project:", String(err));
        toast("Failed to open project:" + String(err));
      }
    }
    init();
  }, [id]);

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
            {fileTree?.children.length && fileTree.name ? (
              <FileTreeSidebar
                fileTree={{
                  ...fileTree!,
                  children: fileTree?.children.filter(
                    (e) => e.name === "assets",
                  )!,
                }}
                projectPath={data?.projectPath!}
              />
            ) : null}
          </ResizablePanel>
          <ResizableHandle />
          <ResizablePanel className="bg-foreground/5">asd</ResizablePanel>
        </ResizablePanelGroup>
      </div>
    </>
  );
}
