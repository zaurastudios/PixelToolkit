import { FileTree } from "@/types/project";
import { ChevronRight, Paintbrush } from "lucide-react";
import React, { useCallback, useEffect, useState } from "react";
import { FixedSizeList as List } from "react-window";
import { Button } from "./ui/button";
import { twMerge } from "tailwind-merge";

type FlattenedEntry = {
  name: string;
  path: string;
  depth: number;
  isExpanded: boolean;
  isMat: boolean;
  hasChildren: boolean;
};

export const FileTreeFolder: React.FC<{
  fileTree: FileTree;
  query: string;
  projectPath: string;
}> = React.memo(({ fileTree, query, projectPath }) => {
  const [expandedNodes, setExpandedNodes] = useState<string[]>([]);
  const [windowHeight, setWindowHeight] = useState(0);

  useEffect(() => {
    if (typeof window !== "undefined") {
      setWindowHeight(window.innerHeight);
    }
  }, []);

  const flattenTree = useCallback(
    (
      nodes: FileTree[],
      depth: number = 0,
      path: string = "",
      result: FlattenedEntry[] = [],
    ): FlattenedEntry[] => {
      nodes.forEach((node) => {
        const currentPath = `${path}/${node.name}`;
        const isExpanded = expandedNodes.includes(currentPath);

        result.push({
          name: node.name,
          path: currentPath,
          depth,
          isExpanded,
          isMat: node.is_mat || false,
          hasChildren: !!node.children?.length,
        });

        if (isExpanded && node.children) {
          flattenTree(node.children, depth + 1, currentPath, result);
        }
      });

      return result;
    },
    [expandedNodes],
  );

  const flattenedTree = React.useMemo(
    () => flattenTree(fileTree.children || []),
    [fileTree, flattenTree],
  );

  const toggleNode = useCallback((path: string) => {
    setExpandedNodes((prev) =>
      prev.includes(path)
        ? prev.filter((node) => node !== path)
        : [...prev, path],
    );
  }, []);

  const rowRenderer = useCallback(
    ({ index, style }: { index: number; style: React.CSSProperties }) => {
      const entry = flattenedTree[index];
      return (
        <div style={style}>
          <EntryRow
            entry={entry}
            toggleNode={toggleNode}
            style={{ marginLeft: `${entry.depth * 16}px` }}
          />
        </div>
      );
    },
    [flattenedTree, toggleNode],
  );

  return (
    <List
      height={windowHeight - (64 + 49)}
      itemCount={flattenedTree.length}
      itemSize={26}
      width="100%"
    >
      {rowRenderer}
    </List>
  );
});

type EntryRowProps = {
  entry: FlattenedEntry;
  toggleNode: (path: string) => void;
  style: React.CSSProperties;
};

const EntryRow: React.FC<EntryRowProps> = React.memo(
  ({ entry, toggleNode, style }) => {
    const dirPath = entry.path + (entry.isMat ? "/mat.yml" : "");

    const handleClick = () => {
      if (!entry.isMat) {
        toggleNode(entry.path);
      } else {
        // Handle material selection if needed
        // window.electron.ipcRenderer.sendMessage("select-texture", entry.path);
      }
    };

    return (
      <div
        className={twMerge("my-1", entry.isMat && "border-l pl-0.5")}
        style={style}
      >
        <Button
          size="sm"
          onClick={handleClick}
          variant={entry.isExpanded ? "secondary" : "ghost"}
          className="h-max px-2 py-1 pl-1 text-left leading-3"
        >
          {entry.isMat && <Paintbrush className="mr-2 size-4 opacity-50" />}
          {!entry.isMat && entry.hasChildren && (
            <ChevronRight
              className={`mr-1 size-4 opacity-50 ${
                entry.isExpanded ? "rotate-90" : undefined
              }`}
            />
          )}
          <span>{entry.name}</span>
        </Button>
      </div>
    );
  },
);
