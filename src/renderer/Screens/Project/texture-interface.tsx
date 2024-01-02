import ProjectConifg from "@/components/project-config";
import * as Resizable from "@/components/ui/resizable";

export default function TextureInterface() {
  return (
    <Resizable.ResizablePanelGroup direction="horizontal">
      <Resizable.ResizablePanel minSize={20} defaultSize={20} maxSize={25}>
        <ProjectConifg />
      </Resizable.ResizablePanel>
      <Resizable.ResizableHandle />
      <Resizable.ResizablePanel />
    </Resizable.ResizablePanelGroup>
  );
}
