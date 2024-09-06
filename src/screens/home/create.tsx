import { ProjectInfo } from "@/types/home";
import { open } from "@tauri-apps/plugin-dialog";

import { Button } from "@/components/ui/button";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Textarea } from "@/components/ui/textarea";
import { invoke } from "@tauri-apps/api/core";
import { Plus } from "lucide-react";
import { useState } from "react";
import { twMerge } from "tailwind-merge";
import { useNavigate } from "react-router-dom";

export const CreateProject = () => {
  const navigate = useNavigate();

  const [dialogOpen, setDialogOpen] = useState(false);
  const [project, setProject] = useState<"new-project" | "import-existing">(
    "new-project",
  );
  const [path, setPath] = useState<string | null>(null);
  const [newProjectInfo, setNewProjectInfo] = useState<ProjectInfo>({
    name: null,
    description: null,
  });
  const [error, setError] = useState<string | null>(null);

  function reset() {
    setProject("new-project");
    setPath("");
    setNewProjectInfo({
      name: null,
      description: null,
    });
  }

  async function openDir() {
    const dir = await open({
      multiple: false,
      directory: true,
      canCreateDirectories: true,
    });
    setPath(dir);
  }

  async function createProject() {
    try {
      const response: string = await invoke("create_project", {
        dirPath: path,
        name: newProjectInfo.name || "My New Project",
        description: newProjectInfo.description,
      });
      const data: CreateProjectResponse = JSON.parse(response);

      if (data.success) {
        reset();
        setDialogOpen(false);
        navigate(`/${data.id}`);
      }
    } catch (error) {
      setError(String(error));
      console.error("Error creating project:", error);
    }
  }

  return (
    <Dialog
      open={dialogOpen}
      onOpenChange={(e) => {
        reset();
        setDialogOpen(e);
      }}
    >
      <DialogTrigger asChild>
        <Card className="flex h-28 cursor-pointer flex-col border-dashed transition-colors hover:bg-foreground/5">
          <CardHeader className="flex grow flex-col justify-center">
            <CardTitle className="flex items-center justify-center gap-2 text-lg font-medium">
              <Plus className="size-7" /> Create Project
            </CardTitle>
          </CardHeader>
        </Card>
      </DialogTrigger>

      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create project</DialogTitle>
        </DialogHeader>

        <Tabs
          value={project}
          onValueChange={(e) => {
            reset();
            setProject(e as "new-project" | "import-existing");
          }}
        >
          <TabsList className="mb-2 grid w-full grid-cols-2">
            <TabsTrigger value="new-project">New Project</TabsTrigger>
            <TabsTrigger value="import-existing">Import Existing</TabsTrigger>
          </TabsList>
          <TabsContent
            value="new-project"
            autoFocus={false}
            className="space-y-2"
            asChild
          >
            <form
              onSubmit={(e) => {
                e.preventDefault();
                createProject();
              }}
            >
              <div className="space-y-1">
                <Label htmlFor="name">
                  Project Name <span className="text-red-600">*</span>
                </Label>
                <Input
                  id="name"
                  placeholder="My New Project"
                  value={newProjectInfo.name || ""}
                  onChange={(e) =>
                    setNewProjectInfo((curr) => ({
                      ...curr,
                      name: e.target.value,
                    }))
                  }
                  required
                />
              </div>
              <div className="space-y-1">
                <Label htmlFor="desc">Project Description</Label>
                <Textarea
                  id="desc"
                  value={newProjectInfo.description || ""}
                  className="max-h-24"
                  onChange={(e) =>
                    setNewProjectInfo((curr) => ({
                      ...curr,
                      description: e.target.value,
                    }))
                  }
                />
              </div>
              <div className="space-y-1">
                <Label htmlFor="path">
                  Path <span className="text-red-600">*</span>
                </Label>
                <Textarea
                  id="path"
                  value={path || undefined}
                  disabled={!!path}
                  required
                  className={twMerge(
                    "h-10 max-h-20 min-h-10",
                    !path && "sr-only",
                  )}
                />
                <Button
                  onClick={openDir}
                  size="sm"
                  variant="secondary"
                  className="flex"
                >
                  {!path ? "Open Folder" : "Change Folder"}
                </Button>
              </div>
              <div>
                <Button type="submit" className="mt-4 w-full">
                  Create Project
                </Button>
              </div>
            </form>
          </TabsContent>
          <TabsContent value="import-existing"></TabsContent>
        </Tabs>
      </DialogContent>
    </Dialog>
  );
};

interface CreateProjectResponse {
  success: boolean;
  id: string | null;
  message: string;
}
