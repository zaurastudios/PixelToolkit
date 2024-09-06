import { Project } from "@/types/home";

import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { Eye, FolderOpen, MoreHorizontal, Trash2 } from "lucide-react";
import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { CreateProject } from "./create";

export function Home() {
  const [projects, setProjects] = useState<Project[]>([]);

  async function getProjects() {
    const pr: Project[] = await invoke("get_projects");
    setProjects(pr);
  }

  const showInFolder = async (path: string) =>
    await invoke("show_in_folder", { path });

  async function removeProject(id: string) {
    const msg = await invoke("remove_project", { id });
    if (msg) getProjects();
  }

  useEffect(() => {
    getProjects();
  }, []);

  return (
    <main className="p-4">
      <div className="grid grid-cols-3 gap-4 lg:grid-cols-4">
        {projects.map((project) => {
          const packPng = project.pack_image
            ? convertFileSrc(project.pack_image)
            : null;

          return (
            <div key={project.id} className="relative">
              <Link to={`/project/${project.id}`}>
                <Card className="flex h-[114px] w-full cursor-pointer flex-col rounded-lg transition-colors hover:bg-foreground/5">
                  <div className="flex grow gap-4">
                    {project.pack_image && (
                      <img
                        src={packPng!}
                        alt={`${project.name}'s pack.png'`}
                        className="m-2 mr-0 aspect-square h-24 w-24 rounded shadow"
                      />
                    )}

                    <CardHeader
                      className={`flex grow flex-col justify-center ${
                        packPng && "pl-0"
                      }`}
                    >
                      <CardTitle className="flex items-center gap-2 text-lg font-medium">
                        {project.name}
                      </CardTitle>

                      {project.description && (
                        <CardDescription className="!mt-0">
                          {`${project.description.slice(0, 30)}${
                            project.description.length > 30 ? "..." : ""
                          }`}
                        </CardDescription>
                      )}
                    </CardHeader>
                  </div>
                </Card>
              </Link>

              <DropdownMenu>
                <DropdownMenuTrigger className="absolute right-2 top-2 z-10 inline-flex h-7 w-7 items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50">
                  <MoreHorizontal className="size-5" />
                </DropdownMenuTrigger>

                <DropdownMenuContent className="w-[200px]" sideOffset={-1}>
                  <DropdownMenuItem
                    className="cursor-pointer text-base"
                    asChild
                  >
                    <Link to={`/project/${project.id}`}>
                      <Eye className="mr-2 size-5" />
                      <span className="text-center text-sm">View Project</span>
                    </Link>
                  </DropdownMenuItem>

                  <DropdownMenuItem
                    className="cursor-pointer text-base"
                    onClick={() => showInFolder(project.path)}
                  >
                    <FolderOpen className="mr-2 size-5" />
                    <span className="text-center text-sm">Show in folder</span>
                  </DropdownMenuItem>

                  <DropdownMenuSeparator />

                  <DropdownMenuItem
                    className="cursor-pointer text-base"
                    onClick={() => removeProject(project.id)}
                  >
                    <Trash2 className="mr-2 size-5 text-red-600" />
                    <span className="text-center text-sm text-red-500">
                      Remove Project
                    </span>
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          );
        })}

        <CreateProject />
      </div>
    </main>
  );
}
