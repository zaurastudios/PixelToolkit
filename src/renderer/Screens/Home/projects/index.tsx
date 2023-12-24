import CreateProject from "./create-project";

export default function Projects() {
  return (
    <div className="p-4 grid rounded-xl border bg-card text-card-foreground shadow-sm grid-cols-4">
      <CreateProject />
    </div>
  );
}
