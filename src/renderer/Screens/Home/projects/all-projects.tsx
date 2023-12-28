import { useEffect, useState } from "react";
import { Skeleton } from "@/components/ui/skeleton";
import * as Card from "@/components/ui/card";
import { motion } from "framer-motion";
import { Link } from "react-router-dom";
import { Config, ProjectFile } from "../../../../main/config-dir";

export default function AllProjects() {
  const [loading, setLoading] = useState(true);
  const [projects, setProjects] = useState<ProjectFile[]>([]);
  useEffect(() => {
    window.electron.ipcRenderer.sendMessage("get-my-projects");
    window.electron.ipcRenderer.on("my-projects", (config) => {
      const { projectFiles } = config as Config;
      setProjects(projectFiles);
      setLoading(false);
    });
  }, []);

  const whileTap = { scale: 0.9 };
  const MotionLinkComponent = motion(Link);

  if (loading) {
    return <Skeleton className="h-full w-full rounded-lg" />;
  }

  if (projects.length === 0) return null;
  console.log(projects);

  return projects.map((project) => (
    <MotionLinkComponent
      key={project.id}
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
  ));
}
