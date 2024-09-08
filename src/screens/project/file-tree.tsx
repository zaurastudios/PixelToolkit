import { FileTreeFolder } from "@/components/file-tree-buttons";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { filterTree } from "@/lib/filter-trees";
import { FileTree } from "@/types/project";
import { TooltipPortal } from "@radix-ui/react-tooltip";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { FolderSync } from "lucide-react";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useDebounce } from "react-use";
import { toast } from "sonner";

export const FileTreeSidebar: React.FC<{
  projectId: string;
}> = ({ projectId }) => {
  const navigate = useNavigate();

  const [data, setData] = useState<{
    fileTree: FileTree;
    redirect: boolean;
    projectPath: string;
  } | null>(null);
  const fileTree = data?.fileTree;
  const projectPath = data?.projectPath;

  const [filteredFileTree, setFilteredFileTree] = useState(fileTree);
  const [query, setQuery] = useState<string>("");

  async function getFileTree() {
    try {
      const res: string = await invoke("get_dirs", { projectId });
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
      console.error(`Failed to open project: ${err}\nID: ${projectId}`);
      toast("Failed to open project:" + String(err));
    }
  }

  useEffect(() => {
    getFileTree();
  }, []);
  useEffect(() => {
    const unlisten = listen<string>("resync_dir_fe", getFileTree);

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  useEffect(() => {
    setFilteredFileTree(fileTree);
  }, [fileTree]);

  useEffect(() => {
    listen<boolean>("unzip-started", (e) => {
      if (!e.payload) getFileTree();
    });
  }, []);

  useDebounce(
    () => {
      if (query) {
        const filteredTree = filterTree(fileTree!, query);
        setFilteredFileTree(filteredTree || fileTree);
      } else {
        setFilteredFileTree(fileTree);
      }
    },
    250,
    [query, fileTree],
  );

  if (!fileTree) return null;

  return (
    <>
      <div className="sticky top-0 z-10 flex gap-1 border-b bg-background p-1">
        <Input
          type="text"
          placeholder="Search..."
          name="search"
          value={query}
          onChange={(e) => setQuery(e.currentTarget.value)}
          className="min-w-none min-w-none z-10 w-auto max-w-none grow"
        />
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button variant="outline" size="icon" onClick={getFileTree}>
                <FolderSync className="size-4" />
              </Button>
            </TooltipTrigger>
            <TooltipPortal>
              <TooltipContent>Resync Folder</TooltipContent>
            </TooltipPortal>
          </Tooltip>
        </TooltipProvider>

        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button variant="outline" size="icon" onClick={getFileTree}>
                <FolderSync className="size-4" />
              </Button>
            </TooltipTrigger>
            <TooltipPortal>
              <TooltipContent>Resync Folder</TooltipContent>
            </TooltipPortal>
          </Tooltip>
        </TooltipProvider>
      </div>

      <FileTreeFolder
        fileTree={{
          ...filteredFileTree!,
          children: filteredFileTree?.children.filter(
            (e) => e.name === "assets",
          )!,
        }}
        projectPath={projectPath!}
      />
    </>
  );
};
