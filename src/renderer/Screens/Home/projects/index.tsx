import AllProjects from "./all-projects";
import CreateProject from "./create-project";

export default function Projects() {
  return (
    <div className="p-4 gap-4 grid rounded-xl border bg-card text-card-foreground shadow-sm grid-cols-4">
      <AllProjects />
      <CreateProject />
    </div>
  );
}
