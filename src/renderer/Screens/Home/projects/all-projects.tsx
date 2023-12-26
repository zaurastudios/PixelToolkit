import { useEffect, useState } from "react";
import { Config, ProjectFile } from "../../../../main/config-dir";
import { Skeleton } from "@/components/ui/skeleton";
import * as Card from "@/components/ui/card";
import { motion } from "framer-motion";
import { Link } from "react-router-dom";

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

  return projects.map((project) => (
    <MotionLinkComponent
      key={project.id}
      whileTap={whileTap}
      to={"/" + project.id}
    >
      <Card.Card className="cursor-pointer hover:bg-foreground/5 transition-colors flex flex-col h-24">
        <Card.CardHeader className="flex flex-col justify-center grow">
          <Card.CardTitle className="flex items-center gap-2 text-lg font-medium">
            {project.name}
          </Card.CardTitle>
          {project.description && (
            <Card.CardDescription className="!mt-0">
              {project.description.slice(0, 40) + "..."}
            </Card.CardDescription>
          )}
        </Card.CardHeader>
      </Card.Card>
    </MotionLinkComponent>
  ));
}
