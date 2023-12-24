import * as Card from "@/components/ui/card";
import { Plus } from "lucide-react";
import { AnimatePresence, motion } from "framer-motion";
import * as Drawer from "@/components/ui/drawer";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useState } from "react";
import Projects from ".";

interface OpenedDirProps {
  cancelled: boolean;
  filePaths: string[];
}

export default function CreateProject() {
  const [path, setPath] = useState<string>("");
  const [cancelled, setCancelled] = useState(false);

  const whileTap = { scale: 0.9 };
  const MotionCardComponent = motion(Card.Card);

  // Handling selecting dir
  function sendOpenDirReq() {
    window.electron.ipcRenderer.sendMessage("open-directory");
  }

  window.electron.ipcRenderer.on("opened-directory", (event) => {
    const { cancelled, filePaths } = event as OpenedDirProps;
    console.log(event);
    if (cancelled) {
      setCancelled(true);
    } else {
      setPath(filePaths[0]);
    }
  });

  return (
    <Drawer.Drawer
      onOpenChange={(e) => {
        if (!e) {
          setPath("");
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

      <Drawer.DrawerContent>
        <Drawer.DrawerHeader>
          <Drawer.DrawerTitle>Create a new project</Drawer.DrawerTitle>
          <Drawer.DrawerDescription>
            You can also import PixelGraph projects
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
              <Button
                onClick={() => sendOpenDirReq()}
                className="w-max"
                variant={!!path && !cancelled ? "outline" : undefined}
              >
                Select Dir
              </Button>
              {cancelled ? (
                <p className="text-red-500 text-sm">
                  Select a folder to continue
                </p>
              ) : (
                <code id="dir-path" className="text-sm text-foreground/80">
                  <em>{path}</em>
                  <Input
                    className="border-0 hidden p-0 max-w-none"
                    disabled
                    value={path}
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
                    <Label htmlFor="project">Project Name</Label>
                    <Input
                      type="text"
                      id="project"
                      name="project"
                      placeholder="My Amazing Project"
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
