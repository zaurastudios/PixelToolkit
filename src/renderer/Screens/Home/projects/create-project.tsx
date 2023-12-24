import * as Card from "@/components/ui/card";
import { Plus } from "lucide-react";
import { motion } from "framer-motion";
import * as Drawer from "@/components/ui/drawer";
import { Button } from "@/components/ui/button";
import * as Tabs from "@/components/ui/tabs";
import { useState } from "react";
import Projects from ".";

type CreateInitSelect = "new" | "existing";
interface OpenedDirProps {
  cancelled: boolean;
  filePaths: string[];
}

export default function CreateProject() {
  const [initSelect, setInitSelect] = useState<CreateInitSelect>("new");

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
    <Drawer.Drawer shouldScaleBackground>
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
          <Tabs.Tabs
            defaultValue="new"
            onValueChange={(e) => setInitSelect(e as CreateInitSelect)}
            className="w-[400px]"
          >
            <Tabs.TabsList className="grid w-full grid-cols-2">
              <Tabs.TabsTrigger value="new">Create New</Tabs.TabsTrigger>
              <Tabs.TabsTrigger value="existing">
                Select Existing
              </Tabs.TabsTrigger>
            </Tabs.TabsList>
            <Tabs.TabsContent value="new">
              <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }}>
                <form className="mt-8 flex flex-col gap-4">
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
                      <code className="text-sm text-foreground/80">
                        <em>{path}</em>
                      </code>
                    )}
                  </div>

                  <div></div>
                </form>
              </motion.div>
            </Tabs.TabsContent>
            <Tabs.TabsContent value="existing">
              <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }}>
                Hiiii
              </motion.div>
            </Tabs.TabsContent>
          </Tabs.Tabs>
        </div>
      </Drawer.DrawerContent>
      <div />
    </Drawer.Drawer>
  );
}
