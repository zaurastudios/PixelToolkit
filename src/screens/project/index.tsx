import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { useParams } from "react-router-dom";
import { FileTreeSidebar } from "./file-tree";

export function ProjectPage() {
  const { id } = useParams();

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
            <FileTreeSidebar projectId={id!} />
          </ResizablePanel>
          <ResizableHandle />
          <ResizablePanel className="bg-foreground/5">asd</ResizablePanel>
        </ResizablePanelGroup>
      </div>
    </>
  );
}
