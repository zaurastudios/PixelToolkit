import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { ProjectFile } from "../../../main/config-dir";

export default function Project() {
  const { id } = useParams();
  const navigate = useNavigate();

  const [loading, setLoading] = useState(true);
  const [projectFile, setProjectFile] = useState<ProjectFile>();
  useEffect(() => {
    window.electron.ipcRenderer.sendMessage("get-project-data-with-id", id!);
    window.electron.ipcRenderer.on("project-data-reply", (arg) => {
      const reply = arg as ProjectFile | false;
      if (!reply) {
        setLoading(false);
        navigate("/");
      } else {
        setProjectFile(reply);
        setLoading(false);
      }
    });
  }, []);

  if (loading) {
    return <h1>Loading...</h1>;
  }

  if (projectFile && projectFile.id) {
    return <div>{projectFile.id}</div>;
  }
}
