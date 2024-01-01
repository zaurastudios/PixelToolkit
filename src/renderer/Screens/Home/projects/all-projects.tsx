import * as Card from "@/components/ui/card";
import * as Dropdown from "@/components/ui/dropdown-menu";
import { Skeleton } from "@/components/ui/skeleton";
import { motion } from "framer-motion";
import { Eye, FolderOpen, MoreHorizontal, Trash2 } from "lucide-react";
import { useEffect, useState } from "react";
import { Link, useLocation, useNavigate } from "react-router-dom";
import { toast } from "sonner";
import { Config, ProjectFile } from "../../../../main/config-dir";

export default function AllProjects() {
  const navigate = useNavigate();
  const location = useLocation();

  const [loading, setLoading] = useState(true);
  const [projects, setProjects] = useState<ProjectFile[]>([]);
  useEffect(() => {
    setLoading(true);
    if (location.pathname === "/") {
      window.electron.ipcRenderer.sendMessage("get-my-projects");
      window.electron.ipcRenderer.on("my-projects", (config) => {
        const { projectFiles } = config as Config;
        setProjects(projectFiles);
        setLoading(false);
      });
    }
  }, [location]);

  const whileTap = { scale: 0.9 };
  const MotionLinkComponent = motion(Link);

  if (loading) {
    return <Skeleton className="h-full w-full rounded-lg" />;
  }

  if (projects.length === 0) return null;

  function openInFolder(path: string) {
    window.electron.ipcRenderer.sendMessage("open-in-folder", path);
  }

  function deleteProject(id: string) {
    window.electron.ipcRenderer.sendMessage("delete-project", id);
  }

  window.electron.ipcRenderer.on("deleted-project", (props) => {
    /* eslint-disable */
    const {
      redirect,
      toast: message,
      error,
    } = props as {
      redirect: string;
      toast: string;
      error?: boolean;
    };
    /* eslint-enable */

    toast(error ? "Error" : "Success", {
      description: message,
    });
    navigate(redirect);
  });

  return projects.map((project) => (
    <div key={project.id} className="relative">
      <MotionLinkComponent
        whileTap={whileTap}
        to={`/${project.id}`}
        className="flex w-full rounded-lg"
      >
        <Card.Card className="flex h-28 w-full cursor-pointer flex-col transition-colors hover:bg-foreground/5">
          <div className="flex grow gap-4">
            {project.packPng && (
              <img
                src={`atom://${project.packPng}`}
                alt={`${project.name}'s pack.png'`}
                className="m-2 mr-0 aspect-square h-24 w-24 rounded-md shadow"
              />
            )}
            <Card.CardHeader
              className={`flex grow flex-col justify-center ${
                project.packPng && "pl-0"
              }`}
            >
              <Card.CardTitle className="flex items-center gap-2 text-lg font-medium">
                {project.name}
              </Card.CardTitle>
              {project.description && (
                <Card.CardDescription className="!mt-0">
                  {`${project.description.slice(0, 30)}${
                    project.description.length > 30 ? "..." : ""
                  }`}
                </Card.CardDescription>
              )}
            </Card.CardHeader>
          </div>
        </Card.Card>
      </MotionLinkComponent>

      <Dropdown.DropdownMenu>
        <Dropdown.DropdownMenuTrigger className="absolute right-2 top-2 z-10 inline-flex h-10 w-10 items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50">
          <MoreHorizontal />
        </Dropdown.DropdownMenuTrigger>

        <Dropdown.DropdownMenuContent className="w-[200px]" sideOffset={-1}>
          <Dropdown.DropdownMenuItem
            className="cursor-pointer text-base"
            asChild
          >
            <Link to={`/${project.id}`}>
              <Eye className="size-5 mr-2" />
              <span>View Project</span>
            </Link>
          </Dropdown.DropdownMenuItem>
          <Dropdown.DropdownMenuItem
            className="cursor-pointer text-base"
            onClick={() => openInFolder(project.path)}
          >
            <FolderOpen className="size-5 mr-2" />
            <span>Open in Folder</span>
          </Dropdown.DropdownMenuItem>
          <Dropdown.DropdownMenuSub>
            <Dropdown.DropdownMenuSubTrigger className="cursor-pointer text-base">
              <Trash2 className="size-5 mr-2 text-red-600" />
              <span className="text-red-500">Delete Project</span>
            </Dropdown.DropdownMenuSubTrigger>

            <Dropdown.DropdownMenuPortal>
              <Dropdown.DropdownMenuSubContent>
                <Dropdown.DropdownMenuLabel>
                  Are you absolutly sure?
                </Dropdown.DropdownMenuLabel>
                <Dropdown.DropdownMenuSeparator />
                <Dropdown.DropdownMenuItem
                  className="cursor-pointer text-base text-red-500"
                  onClick={() => deleteProject(project.id)}
                >
                  <Trash2 className="size-5 mr-2" />
                  <span>Yes</span>
                </Dropdown.DropdownMenuItem>
              </Dropdown.DropdownMenuSubContent>
            </Dropdown.DropdownMenuPortal>
          </Dropdown.DropdownMenuSub>
        </Dropdown.DropdownMenuContent>
      </Dropdown.DropdownMenu>
    </div>
  ));
}
