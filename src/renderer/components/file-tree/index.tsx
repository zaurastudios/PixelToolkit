import { ChevronRight, Paintbrush } from "lucide-react";
import React, { useEffect, useState } from "react";
import { v4 as uuidv4 } from "uuid";
import { FileTreeProps } from "../../../main/ipc-events/project";
import { Button } from "../ui/button";

export const FileTreeFolder = React.memo(
  (props: { fileTree: FileTreeProps; query: string; projectPath: string }) => {
    const { fileTree, query, projectPath } = props;
    const [expandedNodes, setExpandedNodes] = useState<string[]>([]);

    return (
      <div className="p-2">
        {fileTree.children?.map((entry) => (
          <Entry
            key={uuidv4()}
            entry={entry}
            depth={1}
            query={query}
            path={projectPath}
            expandedNodes={expandedNodes}
            setExpandedNodes={setExpandedNodes}
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
  expandedNodes: string[];
  setExpandedNodes: React.Dispatch<React.SetStateAction<string[]>>;
};

export function Entry({
  entry,
  depth,
  query,
  path,
  expandedNodes,
  setExpandedNodes,
}: EntryProps) {
  const [isExpanded, setIsExpanded] = useState<boolean>(
    expandedNodes.includes(`${path}/${entry.name}`),
  );

  const currentPath = `${path}/${entry.name}`;

  function selectTexture() {
    window.electron.ipcRenderer.sendMessage("select-texture", currentPath);
  }

  function handleNodeExpansion() {
    if (entry.isMat) {
      selectTexture();
    } else {
      setIsExpanded((prev) => !prev);
      if (isExpanded) {
        setExpandedNodes((prev) => prev.filter((node) => node !== currentPath));
      } else {
        setExpandedNodes((prev) => [...prev, currentPath]);
      }
    }
  }

  return (
    <>
      <Button
        data-label="filefolder"
        // eslint-disable-next-line
        onClick={handleNodeExpansion}
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
                expandedNodes={expandedNodes}
                setExpandedNodes={setExpandedNodes}
              />
            ))}
          </div>
        )}
      </div>
    </>
  );
}
