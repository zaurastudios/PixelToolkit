import * as Card from "@/components/ui/card";
import { Plus } from "lucide-react";
import { AnimatePresence, motion } from "framer-motion";
import * as Drawer from "@/components/ui/drawer";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useEffect, useState } from "react";
import Projects from ".";

interface OpenedProps {
  canceled: boolean;
  filePaths: string[];
}

export default function CreateProject() {
  const [path, setPath] = useState<string>("");
  const [ymlPath, setYmlPath] = useState<string>("");
  const [cancelled, setCancelled] = useState(false);

  const whileTap = { scale: 0.9 };
  const MotionCardComponent = motion(Card.Card);

  // Handling selecting dir
  function sendOpenDirReq() {
    setYmlPath("");
    setCancelled(false);
    window.electron.ipcRenderer.sendMessage("open-directory");
  }

  window.electron.ipcRenderer.on("opened-directory", (event) => {
    const { canceled, filePaths } = event as OpenedProps;
    console.log(event);
    if (canceled) {
      setCancelled(true);
    } else {
      setYmlPath(filePaths[0]);
    }
  });

  // Handling selecting existisng project .yml
  function sendSelectYmlReq() {}

  window.electron.ipcRenderer.on("opened-yml", (event) => {
    const { canceled, filePaths } = event as OpenedProps;
    if (canceled) {
      setCancelled(true);
    } else {
      setYmlPath(filePaths[0]);
    }
  });

  return (
    <Drawer.Drawer
      onOpenChange={(e) => {
        if (!e) {
          setPath("");
          setYmlPath("");
          setCancelled(false);
        }
      }}
      shouldScaleBackground
    >
      <Drawer.DrawerTrigger asChild>
        <MotionCardComponent
          whileTap={whileTap}
          className="cursor-pointer hover:bg-foreground/5 transition-colors"
        >
          <Card.CardHeader>
            <Card.CardTitle className="flex items-center gap-2 text-lg font-medium">
              <Plus height={20} /> Create new project
            </Card.CardTitle>
          </Card.CardHeader>
        </MotionCardComponent>
      </Drawer.DrawerTrigger>
      <Drawer.DrawerContent className="h-full max-h-[70%] mt-32">
        <Drawer.DrawerHeader>
          <Drawer.DrawerTitle>Create a new project</Drawer.DrawerTitle>
          <Drawer.DrawerDescription>
            You can also import PixelGraph projects. Start by clicking on the
            Select Directory button.
          </Drawer.DrawerDescription>
        </Drawer.DrawerHeader>

        <div className="p-4">
          <form
            onSubmit={(e) => {
              e.preventDefault();
            }}
            className="mt-2 flex flex-col gap-4"
          >
            <div className="flex items-center gap-4">
              <div className="flex gap-2 items-center">
                <Button
                  onClick={() => sendOpenDirReq()}
                  className="w-max"
                  variant={!!path && !cancelled ? "outline" : undefined}
                >
                  Select Directory
                </Button>
                <Button
                  onClick={() => sendSelectYmlReq()}
                  className="w-max"
                  variant={!!ymlPath && !cancelled ? "outline" : "secondary"}
                >
                  Select Existing Project
                </Button>
              </div>
              {cancelled && (
                <p className="text-red-500 text-sm">
                  Select a folder/project.yml to continue
                </p>
              )}
              {!!path && (
                <code id="dir-path" className="text-sm text-foreground/80">
                  <em>{path}</em>
                  <Input
                    className="border-0 hidden p-0 max-w-none"
                    disabled
                    name="dir-path"
                    value={path}
                    aria-hidden
                  />
                </code>
              )}
              {!!ymlPath && (
                <code id="yml-path" className="text-sm text-foreground/80">
                  <em>{path}</em>
                  <Input
                    className="border-0 hidden p-0 max-w-none"
                    disabled
                    name="yml-path"
                    value={ymlPath}
                    aria-hidden
                  />
                </code>
              )}
            </div>

            {!!path && !cancelled && (
              <AnimatePresence>
                <motion.div
                  key="show-once-path-is-selected"
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  exit={{ opacity: 0 }}
                  className="flex flex-col gap-4"
                >
                  <div className="grid w-full max-w-sm items-center gap-1.5">
                    <Label aria-required htmlFor="project">
                      Project Name{" "}
                      <span className="text-red-500" aria-hidden>
                        *
                      </span>
                    </Label>
                    <Input
                      type="text"
                      id="project"
                      name="project"
                      placeholder="My Amazing Project"
                      required
                    />
                  </div>
                </motion.div>
              </AnimatePresence>
            )}
          </form>
        </div>
      </Drawer.DrawerContent>
      <div />
    </Drawer.Drawer>
  );
}
