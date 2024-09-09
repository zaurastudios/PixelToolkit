import { ProjectInfo } from "@/types/home";

import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Plus } from "lucide-react";
import { useState } from "react";
import { NewProject } from "./new";
import { ImportExisting } from "./existing";

export const CreateProject = () => {
  const [dialogOpen, setDialogOpen] = useState(false);
  const [project, setProject] = useState<"new-project" | "import-existing">(
    "new-project",
  );

  const [path, setPath] = useState<string | null>(null);
  const [newProjectInfo, setNewProjectInfo] = useState<ProjectInfo>({
    name: null,
    description: null,
    createMcDirs: false,
    createRealmsDirs: false,
    createOfDirs: false,
  });
  const [error, setError] = useState<string | null>(null);
  const [zipPath, setZipPath] = useState<string | null>(null);

  const [projectYmlPath, setProjectYmlPath] = useState<string | null>(null);

  function reset() {
    setProject("new-project");
    setPath("");
    setNewProjectInfo({
      name: null,
      description: null,
      createMcDirs: false,
      createRealmsDirs: false,
      createOfDirs: false,
    });
    setError("");
    setZipPath("");
    setProjectYmlPath("");
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
            <NewProject
              reset={reset}
              setDialogOpen={setDialogOpen}
              newProjectInfo={newProjectInfo}
              setNewProjectInfo={setNewProjectInfo}
              path={path}
              setPath={setPath}
              error={error}
              setError={setError}
              zipPath={zipPath}
              setZipPath={setZipPath}
            />
          </TabsContent>
          <TabsContent
            value="import-existing"
            autoFocus={false}
            className="space-y-2"
            asChild
          >
            <ImportExisting
              reset={reset}
              setDialogOpen={setDialogOpen}
              error={error}
              setError={setError}
              projectYmlPath={projectYmlPath}
              setProjectYmlPath={setProjectYmlPath}
            />
          </TabsContent>
        </Tabs>
      </DialogContent>
    </Dialog>
  );
};
