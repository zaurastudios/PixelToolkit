import { ProjectInfo } from "@/types/home";

import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { twMerge } from "tailwind-merge";

const MAX_STEPS = 4;

export const NewProject = ({
  reset,
  setDialogOpen,
  newProjectInfo,
  setNewProjectInfo,
  path,
  setPath,
  error,
  setError,
}: NewProjectProps) => {
  const navigate = useNavigate();

  const [step, setStep] = useState<number>(1);

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
        createMcDirs: newProjectInfo.createMcDirs,
        createRealmsDirs: newProjectInfo.createRealmsDirs,
        createOfDirs: newProjectInfo.createOfDirs,
      });
      const data: CreateProjectResponse = JSON.parse(response);

      if (data.success) {
        reset();
        setDialogOpen(false);
        navigate(`/project/${data.id}`);
      } else {
        setError(data.message);
      }
    } catch (error) {
      setError(String(error));
      console.error("Error creating project:", error);
    }
  }

  const nextDisabledConditons = step === MAX_STEPS || !path;

  return (
    <>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          createProject();
        }}
      >
        {error && (
          <div className="mb-1">
            <div className="mb-1 rounded-md border border-red-700/30 bg-red-500/20 p-2">
              {error}
            </div>
          </div>
        )}

        {step === 1 && (
          <>
            <div className="space-y-1">
              <Label htmlFor="name">Project Name</Label>
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
                  "h-10 max-h-20 min-h-10 w-max",
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
          </>
        )}

        {step === 2 && (
          <>
            <div className="mt-4 space-y-3">
              <div className="flex items-center space-x-2">
                <Checkbox
                  id="mc"
                  checked={newProjectInfo.createMcDirs}
                  onCheckedChange={(e) =>
                    setNewProjectInfo({
                      ...newProjectInfo,
                      createMcDirs: !!e,
                    })
                  }
                />
                <label
                  htmlFor="mc"
                  className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                >
                  Create Minecraft folders
                </label>
              </div>
              <div className="flex items-center space-x-2">
                <Checkbox
                  id="realms"
                  checked={newProjectInfo.createRealmsDirs}
                  onCheckedChange={(e) =>
                    setNewProjectInfo({
                      ...newProjectInfo,
                      createRealmsDirs: !!e,
                    })
                  }
                />
                <label
                  htmlFor="realms"
                  className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                >
                  Create realms folders
                </label>
              </div>
              <div className="flex items-center space-x-2">
                <Checkbox
                  id="of"
                  checked={newProjectInfo.createOfDirs}
                  onCheckedChange={(e) =>
                    setNewProjectInfo({
                      ...newProjectInfo,
                      createOfDirs: !!e,
                    })
                  }
                />
                <label
                  htmlFor="of"
                  className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                >
                  Create OptiFine folders
                </label>
              </div>
            </div>
          </>
        )}

        {path && step === MAX_STEPS && (
          <div>
            <Button type="submit" className="mt-4 w-full">
              Create Project
            </Button>
          </div>
        )}
      </form>

      <div className="mt-3 flex justify-between gap-2">
        <div>
          {step !== 1 && (
            <Button
              variant="ghost"
              disabled={step === 1}
              onClick={() => setStep(step - 1)}
            >
              Back
            </Button>
          )}
        </div>
        <div>
          {step !== 4 && (
            <Button
              variant="ghost"
              disabled={nextDisabledConditons}
              onClick={() => setStep(step + 1)}
            >
              Next
            </Button>
          )}
        </div>
      </div>
    </>
  );
};

interface CreateProjectResponse {
  success: boolean;
  id: string | null;
  message: string;
}

interface NewProjectProps {
  reset: () => void;
  setDialogOpen: React.Dispatch<React.SetStateAction<boolean>>;

  newProjectInfo: ProjectInfo;
  setNewProjectInfo: React.Dispatch<React.SetStateAction<ProjectInfo>>;

  path: string | null;
  setPath: React.Dispatch<React.SetStateAction<string | null>>;

  error: string | null;
  setError: React.Dispatch<React.SetStateAction<string | null>>;
}
