import React, { useEffect, useState } from "react";
import { Skeleton } from "@/components/ui/skeleton";
import * as Card from "@/components/ui/card";
import { motion } from "framer-motion";
import { Link, useLocation, useNavigate } from "react-router-dom";
import * as Dropdown from "@/components/ui/dropdown-menu";
import { Eye, FolderOpen, MoreHorizontal, Trash2 } from "lucide-react";
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
        className="rounded-lg w-full flex"
      >
        <Card.Card className="cursor-pointer w-full hover:bg-foreground/5 transition-colors flex flex-col h-28">
          <div className="flex grow gap-4">
            {project.packPng && (
              <img
                src={`atom://${project.packPng}`}
                alt={`${project.name}'s pack.png'`}
                className="aspect-square h-24 rounded-md shadow m-2 mr-0 w-24"
              />
            )}
            <Card.CardHeader
              className={`flex flex-col justify-center grow ${
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
        <Dropdown.DropdownMenuTrigger className="absolute top-2 right-2 inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 w-10 z-10">
          <MoreHorizontal />
        </Dropdown.DropdownMenuTrigger>

        <Dropdown.DropdownMenuContent className="w-[200px]" sideOffset={-1}>
          <Dropdown.DropdownMenuItem
            className="cursor-pointer text-base"
            asChild
          >
            <Link to={`/${project.id}`}>
              <Eye className="mr-2 size-5" />
              <span>View Project</span>
            </Link>
          </Dropdown.DropdownMenuItem>
          <Dropdown.DropdownMenuItem
            className="cursor-pointer text-base"
            onClick={() => openInFolder(project.path)}
          >
            <FolderOpen className="mr-2 size-5" />
            <span>Open in Folder</span>
          </Dropdown.DropdownMenuItem>
          <Dropdown.DropdownMenuSub>
            <Dropdown.DropdownMenuSubTrigger className="cursor-pointer text-base">
              <Trash2 className="mr-2 size-5 text-red-600" />
              <span className="text-red-500">Delete Project</span>
            </Dropdown.DropdownMenuSubTrigger>

            <Dropdown.DropdownMenuPortal>
              <Dropdown.DropdownMenuSubContent>
                <Dropdown.DropdownMenuLabel>
                  Are you absolutly sure?
                </Dropdown.DropdownMenuLabel>
                <Dropdown.DropdownMenuSeparator />
                <Dropdown.DropdownMenuItem
                  className="cursor-pointer text-red-500 text-base"
                  onClick={() => deleteProject(project.id)}
                >
                  <Trash2 className="mr-2 size-5" />
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
