import * as Card from "@/components/ui/card";
import { Plus } from "lucide-react";
import { motion } from "framer-motion";
import * as Drawer from "@/components/ui/drawer";
import { Button } from "@/components/ui/button";
import React, { useState } from "react";
import { useToast } from "@/components/ui/use-toast";
import { useNavigate } from "react-router-dom";
import NewProjectContent from "./new-project";
import SelectProject from "./select-project";

interface OpenedProps {
  canceled: boolean;
  filePaths: string[];
  isNotEmpty?: boolean;
}

export default function CreateProject() {
  const { toast } = useToast();
  const navigate = useNavigate();

  const [open, setOpen] = useState(false);

  const [path, setPath] = useState<string>("");
  const [ymlPath, setYmlPath] = useState<string>("");
  const [cancelled, setCancelled] = useState(false);
  const [empty, setEmpty] = useState(true);

  const whileTap = { scale: 0.9 };
  const MotionCardComponent = motion(Card.Card);

  // Handling selecting dir
  function reset() {
    setPath("");
    setYmlPath("");
    setCancelled(false);
    setEmpty(true);
  }

  function sendOpenDirReq() {
    reset();
    window.electron.ipcRenderer.sendMessage("open-directory");
  }

  window.electron.ipcRenderer.on("opened-directory", (event) => {
    const { canceled, filePaths, isNotEmpty } = event as OpenedProps;
    if (canceled) {
      setCancelled(true);
    } else if (isNotEmpty) {
      setEmpty(false);
    } else {
      setPath(filePaths[0]);
    }
  });

  // Handling selecting existisng project .yml
  function sendSelectYmlReq() {
    reset();
    window.electron.ipcRenderer.sendMessage("open-yml");
  }

  window.electron.ipcRenderer.on("opened-yml", (event) => {
    const { canceled, filePaths } = event as OpenedProps;
    if (canceled) {
      setCancelled(true);
    } else {
      setYmlPath(filePaths[0]);
    }
  });

  function submitHandler(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    const formData = new FormData(e.currentTarget);
    const projectTitle = formData.get("project")!;
    const projectDescription = formData.get("description") ?? "";

    if (ymlPath) {
      const data = {
        ymlPath,
        projectTitle,
        projectDescription,
      };
      window.electron.ipcRenderer.sendMessage("add-existing-project", data);
    }

    if (path) {
      const data = {
        path,
        projectTitle,
        projectDescription,
      };
      window.electron.ipcRenderer.sendMessage("create-project-in-dir", data);
    }
  }

  // Handling create success
  window.electron.ipcRenderer.on("created", (arg) => {
    const { id } = arg as { id: string };
    navigate(`/${id}` as string);
  });

  // Handling create errors
  window.electron.ipcRenderer.on("error-create", (event) => {
    if (event) {
      toast({ title: event as string, variant: "destructive" });
    }
  });

  return (
    <Drawer.Drawer
      onOpenChange={(e) => {
        if (!e) reset();
        setOpen(e);
      }}
      open={open}
      shouldScaleBackground
    >
      <Drawer.DrawerTrigger asChild>
        <MotionCardComponent
          whileTap={whileTap}
          className="cursor-pointer flex flex-col h-24 hover:bg-foreground/5 transition-colors"
        >
          <Card.CardHeader className="flex flex-col grow justify-center">
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
          <form onSubmit={submitHandler} className=" flex mt-2 flex-col gap-4">
            <div className="flex items-center gap-4">
              <div className="flex gap-2 items-center">
                <Button
                  onClick={() => sendOpenDirReq()}
                  className="w-max"
                  variant={path ? "outline" : undefined}
                  type="button"
                >
                  Select Directory
                </Button>
                <Button
                  onClick={() => sendSelectYmlReq()}
                  className="w-max"
                  variant={ymlPath ? "outline" : "secondary"}
                  type="button"
                >
                  Select Existing Project
                </Button>
              </div>
              {cancelled && (
                <p className="text-red-500 text-sm">
                  Select a folder/project.yml to continue
                </p>
              )}
              {!empty && (
                <p className="text-red-500 text-sm">
                  Please choose an empty directory
                </p>
              )}
              {path && (
                <code id="dir-path" className="text-sm text-foreground/80">
                  <em>{path}</em>
                  <input
                    type="text"
                    className="hidden"
                    readOnly
                    name="dir-path"
                    value={path}
                    aria-hidden
                  />
                </code>
              )}
              {ymlPath && (
                <code id="yml-path" className="text-sm text-foreground/80">
                  <em>{ymlPath}</em>
                  <input
                    type="text"
                    className="hidden"
                    name="yml-path"
                    value={ymlPath}
                    readOnly
                    aria-hidden
                  />
                </code>
              )}
            </div>
            {path && <NewProjectContent />}
            {ymlPath && <SelectProject ymlPath={ymlPath} />}
          </form>
        </div>
      </Drawer.DrawerContent>
      <div />
    </Drawer.Drawer>
  );
}
