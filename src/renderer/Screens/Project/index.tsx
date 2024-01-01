import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import * as Resizable from "@/components/ui/resizable";
import { useToast } from "@/components/ui/use-toast";
import { ProjectFile } from "../../../main/config-dir";
import { FileTreeProps } from "../../../main/ipc-events/project";
import Sidebar from "./sidebar";
import TextureInterface from "./texture-interface";

type FileTreeReply =
  | { redirect: false; fileTree: FileTreeProps }
  | { redirect: true; message: string };

export default function Project() {
  const { id } = useParams();
  const navigate = useNavigate();
  const { toast } = useToast();

  const [loading, setLoading] = useState(true);
  const [projectFile, setProjectFile] = useState<ProjectFile>();
  const [fileTree, setFileTree] = useState<FileTreeProps>();

  useEffect(() => {
    window.electron.ipcRenderer.sendMessage("get-project-data-with-id", id!);
    window.electron.ipcRenderer.on("project-data-reply", (arg) => {
      const reply = arg as ProjectFile | false;
      if (!reply) {
        setLoading(false);
        navigate("/");
      } else {
        setProjectFile(reply);
        window.electron.ipcRenderer.sendMessage(
          "set-title",
          `${reply.name} | PixelToolkit`,
        );

        window.electron.ipcRenderer.sendMessage(
          "get-project-file-tree",
          reply.path,
        );
        window.electron.ipcRenderer.on("project-file-tree", (treeArg) => {
          const treeReply = treeArg as FileTreeReply;
          if (treeReply.redirect) {
            toast({
              description: treeReply.message,
            });
            navigate("/");
          } else {
            setFileTree(treeReply.fileTree);
          }
        });
        setLoading(false);
      }
    });

    // eslint-disable-next-line
  }, [id, navigate]);

  if (loading) {
    return <h1>Loading...</h1>;
  }

  if (projectFile && projectFile.id) {
    return (
      <div className="h-[calc(100vh-56px-32px)] p-4">
        <Resizable.ResizablePanelGroup
          direction="horizontal"
          className="rounded-xl border bg-card text-card-foreground shadow-sm"
        >
          <Resizable.ResizablePanel
            minSize={15}
            defaultSize={15}
            maxSize={20}
            className="!overflow-scroll"
          >
            {fileTree?.name && (
              <Sidebar fileTree={fileTree} projectPath={projectFile.path} />
            )}
          </Resizable.ResizablePanel>
          <Resizable.ResizableHandle />
          <Resizable.ResizablePanel>
            <TextureInterface />
          </Resizable.ResizablePanel>
        </Resizable.ResizablePanelGroup>
      </div>
    );
  }
}
