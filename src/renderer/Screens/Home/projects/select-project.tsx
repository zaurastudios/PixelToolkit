import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@radix-ui/react-menubar";
import { useEffect, useState } from "react";

interface Props {
  ymlPath: string;
}

interface IsPresentProps {
  isPresent: boolean;
  message?: string;
}

interface YmlData {
  name: string;
  description: string;
}

export default function SelectProject({ ymlPath }: Props) {
  const [isPresent, setIsPresent] = useState(false);
  const [message, setMessage] = useState("");
  const [ymlData, setYmlData] = useState<YmlData | false>();

  useEffect(() => {
    if (ymlPath) {
      window.electron.ipcRenderer.sendMessage(
        "selected-path-in-config",
        ymlPath,
      );

      window.electron.ipcRenderer.on("selected-path-is-in-config", (props) => {
        /* eslint-disable react/prop-types */
        const { isPresent: propsIsPresent, message: propsMessage } =
          props as IsPresentProps;

        setIsPresent(propsIsPresent);
        if (propsMessage) {
          setMessage(propsMessage);
        }
      });
    }
  }, [ymlPath]);

  useEffect(() => {
    if (!isPresent) {
      window.electron.ipcRenderer.sendMessage("get-yml-data", ymlPath);
    }
  }, [isPresent, ymlPath]);

  window.electron.ipcRenderer.on("yml-data", (props) => {
    setYmlData(props as YmlData | false);
  });

  return (
    <>
      <div />

      {isPresent && message && (
        <p
          className="text-red-500"
          dangerouslySetInnerHTML={{ __html: message }}
        />
      )}

      {!ymlData && (
        <p className="text-red-500">
          Please select a valid <code>project.yml</code> file
        </p>
      )}

      {!isPresent && ymlData && (
        <>
          <div className="flex gap-4">
            <div className="grid w-full max-w-sm items-center gap-1.5">
              <Label aria-required>
                Project Name{" "}
                <span className="text-red-500" aria-hidden>
                  *
                </span>
              </Label>
              <Input
                type="text"
                name="project"
                placeholder="My Amazing Project"
                required
                defaultValue={ymlData?.name}
              />
            </div>
            <div className="grid w-full max-w-sm items-center gap-1.5">
              <Label aria-required>Project Description </Label>
              <Input
                type="text"
                name="description"
                defaultValue={ymlData?.description}
                placeholder="My amazing project description"
              />
            </div>
          </div>
          <Button className="w-max" type="submit">
            Add
          </Button>
        </>
      )}
    </>
  );
}
