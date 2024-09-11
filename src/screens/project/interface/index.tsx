import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import ProjectConfig from "./project-config";
import { TextureCanvas } from "@/components/texture-canvas";

export function Interface({ projectPath }: { projectPath: string }) {
  return (
    <>
      <ResizablePanelGroup direction="horizontal">
        <ResizablePanel minSize={20} defaultSize={20} maxSize={25}>
          <ProjectConfig texturePath={projectPath} />
        </ResizablePanel>
        <ResizableHandle />
        <ResizablePanel className="bg-foreground/[0.02]">
          <TextureCanvas />
        </ResizablePanel>
      </ResizablePanelGroup>
    </>
  );
}
