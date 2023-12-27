import { useEffect } from "react";

interface Props {
  ymlPath: string;
}

export default function SelectProject({ ymlPath }: Props) {
  useEffect(() => {
    if (ymlPath) {
      window.electron.ipcRenderer.sendMessage("selected-path-in-config");
    }
  }, [ymlPath]);

  return <h1>1ads</h1>;
}
