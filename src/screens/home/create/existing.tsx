import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useNavigate } from "react-router-dom";
import { twMerge } from "tailwind-merge";

export const ImportExisting = ({
  reset,
  setDialogOpen,
  error,
  setError,
  projectYmlPath,
  setProjectYmlPath,
}: ExistingProjectProps) => {
  const navigate = useNavigate();

  async function setProjectYml() {
    const dir = await open({
      multiple: false,
      directory: false,
      canCreateDirectories: true,
      filters: [
        {
          name: "Choose zip archive",
          extensions: ["yml"],
        },
      ],
    });
    setProjectYmlPath(dir);
  }

  async function addProject() {
    try {
      const response: string = await invoke("create_project_existing", {
        projectYmlPath,
      });
      const data = await JSON.parse(response);

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

  return (
    <>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          addProject();
        }}
      >
        {error && (
          <div className="mb-1">
            <div className="mb-1 rounded-md border border-red-700/30 bg-red-500/20 p-2">
              {error}
            </div>
          </div>
        )}

        <div className="space-y-1">
          <Label htmlFor="path">
            Path <span className="text-red-600">*</span>
          </Label>
          <Textarea
            id="path"
            value={projectYmlPath || undefined}
            disabled={!!projectYmlPath}
            required
            className={twMerge(
              "h-10 max-h-20 min-h-10 w-full max-w-[462px]",
              !projectYmlPath && "sr-only",
            )}
          />
          <Button
            onClick={setProjectYml}
            size="sm"
            variant="secondary"
            className="flex"
            type="button"
          >
            {!projectYmlPath ? "Open Project" : "Change Project"}
          </Button>
        </div>

        <div>
          <Button
            type="submit"
            className="mt-4 w-full"
            disabled={!projectYmlPath}
          >
            Create Project
          </Button>
        </div>
      </form>
    </>
  );
};

interface ExistingProjectProps {
  reset: () => void;
  setDialogOpen: React.Dispatch<React.SetStateAction<boolean>>;

  error: string | null;
  setError: React.Dispatch<React.SetStateAction<string | null>>;

  projectYmlPath: string | null;
  setProjectYmlPath: React.Dispatch<React.SetStateAction<string | null>>;
}
