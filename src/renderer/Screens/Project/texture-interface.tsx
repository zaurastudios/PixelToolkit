import TwoDCanvas from "@/components/2d-canvas";
import ProjectConfig from "@/components/project-config";
import * as Resizable from "@/components/ui/resizable";

export default function TextureInterface({
  texturePath,
}: {
  texturePath: string;
}) {
  return (
    <Resizable.ResizablePanelGroup direction="horizontal">
      <Resizable.ResizablePanel minSize={20} defaultSize={20} maxSize={25}>
        <ProjectConfig texturePath={texturePath} />
      </Resizable.ResizablePanel>
      <Resizable.ResizableHandle />
      <Resizable.ResizablePanel className="bg-foreground/5">
        <TwoDCanvas />
      </Resizable.ResizablePanel>
    </Resizable.ResizablePanelGroup>
  );
}
