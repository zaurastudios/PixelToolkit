import { ChevronRight, Paintbrush } from "lucide-react";
import React, { useState } from "react";
import { v4 as uuidv4 } from "uuid";
import { FileTreeProps } from "../../../main/ipc-events/project";
import { Button } from "../ui/button";

export const FileTreeFolder = React.memo(
  (props: { fileTree: FileTreeProps; query: string; projectPath: string }) => {
    const { fileTree, query, projectPath } = props;

    return (
      <div className="p-2">
        {fileTree.children?.map((entry) => (
          // eslint-disable-next-line
          <Entry
            key={uuidv4()}
            entry={entry}
            depth={1}
            query={query}
            path={projectPath}
          />
        ))}
      </div>
    );
  },
);

// Article referred: https://blog.stackademic.com/creating-a-folder-tree-view-component-with-react-and-typescript-d3df2086878e
type EntryProps = {
  entry: FileTreeProps;
  depth: number;
  query: string;
  path: string;
};

export const Entry = React.memo(({ entry, depth, query, path }: EntryProps) => {
  const [isExpanded, setIsExpanded] = useState<boolean>(
    query.length > 2 && !entry.isMat,
  );

  const currentPath = `${path}/${entry.name}`;

  function selectTexture() {
    window.electron.ipcRenderer.sendMessage("select-texture", currentPath);
  }

  return (
    <>
      <Button
        data-label="filefolder"
        onClick={() => {
          if (!entry.isMat) {
            setIsExpanded((prev) => !prev);
          } else {
            selectTexture();
          }
        }}
        variant={isExpanded ? "secondary" : "ghost"}
        className="h-max px-2 py-2 pr-3 leading-3"
        data-path={currentPath + (entry.isMat ? "/mat.yml" : "")}
      >
        {entry.isMat && <Paintbrush className="size-4 mr-2 opacity-50" />}
        {!entry.isMat && entry.children && (
          <div>
            <ChevronRight
              className={`mr-2 size-4 opacity-50 ${
                isExpanded ? "rotate-90" : undefined
              }`}
            />
          </div>
        )}
        <span>{entry.name}</span>
      </Button>

      <div className="ml-1 mt-1 border-l">
        {isExpanded && (
          <div className="ml-1">
            {entry.children?.map((e) => (
              <Entry
                key={uuidv4()}
                entry={e}
                depth={depth + 1}
                query={query}
                path={currentPath}
              />
            ))}
          </div>
        )}
      </div>
    </>
  );
});
