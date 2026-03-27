import { useQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import { getProjects } from "./actions";
import { PlusIcon, SpinnerGapIcon } from "@phosphor-icons/react";

export const Route = createFileRoute("/")({
  component: Index,
});

function Index() {
  const {
    isPending,
    error,
    data: projects,
  } = useQuery({
    queryKey: ["getProjects"],
    queryFn: getProjects,
  });

  if (isPending) {
    return (
      <div className="flex h-[calc(100vh-32px)] flex-col justify-between">
        <div className="grid h-[calc(100vh-64px)] place-items-center p-4">
          <h1 className="flex gap-4 font-mono text-2xl font-semibold select-none">
            <SpinnerGapIcon className="size-6 translate-y-0.5 animate-spin" />
            Loading...
          </h1>
        </div>
        <footer className="text-muted-foreground flex h-8 items-center justify-between border-t px-2.5">
          <strong className="font-mono text-xs font-medium">
            PixelToolkit
          </strong>
          <span className="text-xs">
            By{" "}
            <a href="https://zaura.net" target="_blank" className="underline">
              Zaura Studios
            </a>
          </span>
        </footer>
      </div>
    );
  }

  return (
    <div className="flex h-[calc(100vh-32px)] flex-col justify-between">
      <div>
        {(error || !projects) && (
          <div className="bg-destructive/10 border-b p-4">
            <h1>
              Error:{" "}
              {!projects ? "Error: Projects are undefined" : String(error)}
            </h1>
          </div>
        )}

        <div className="grid max-h-[calc(100vh-64px)] overflow-y-auto border-l sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
          {projects &&
            projects.map((project) => (
              <button
                key={project.id}
                className="group flex items-center justify-center gap-2.5 border-r border-b p-5"
              >
                {project.name}
              </button>
            ))}
          <button className="group flex items-center justify-center gap-2.5 border-r border-b p-5">
            <PlusIcon className="-translate-y-px opacity-50 transition-opacity group-hover:opacity-100" />
            Create Project
          </button>
        </div>
      </div>
      <footer className="text-muted-foreground flex h-8 items-center justify-between border-t px-2.5">
        <strong className="font-mono text-xs font-medium">PixelToolkit</strong>
        <span className="text-xs">
          By{" "}
          <a href="https://zaura.net" target="_blank" className="underline">
            Zaura Studios
          </a>
        </span>
      </footer>
    </div>
  );
}
