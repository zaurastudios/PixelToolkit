import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/context-menu";
import { FileTree } from "@/types/project";
import { invoke } from "@tauri-apps/api/core";
import { ChevronRight, FolderOpen, Paintbrush } from "lucide-react";
import React, { useCallback, useEffect, useState } from "react";
import { FixedSizeList as List } from "react-window";
import { twMerge } from "tailwind-merge";
import { Button } from "./ui/button";
import { buildPath } from "@/lib/utils";
import { toast } from "sonner";

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
  projectPath: string;
}> = React.memo(({ fileTree, projectPath }) => {
  const [expandedNodes, setExpandedNodes] = useState<string[]>([]);
  const [windowHeight, setWindowHeight] = useState(0);

  useEffect(() => {
    if (typeof window !== "undefined") {
      setWindowHeight(window.innerHeight);
      window.addEventListener("resize", () =>
        setWindowHeight(window.innerHeight),
      );

      return () =>
        window.removeEventListener("resize", () =>
          setWindowHeight(window.innerHeight),
        );
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
        // const isExpanded = true;

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

  const [selectedMaterial, setSelectedMaterial] = useState("");

  async function selectMaterial(materialPath: string) {
    try {
      if (materialPath !== selectedMaterial) {
        const res = await invoke("select_texture", {
          materialPath,
        });
        setSelectedMaterial(String(res));
      }
    } catch (err) {
      toast("Failed to select texture: " + String(err));
      console.error("Failed to select texture: ", String(err));
    }
  }

  const rowRenderer = useCallback(
    ({ index, style }: { index: number; style: React.CSSProperties }) => {
      const entry = flattenedTree[index];
      return (
        <div style={style}>
          <EntryRow
            entry={entry}
            toggleNode={toggleNode}
            projectPath={projectPath}
            selectMaterial={selectMaterial}
            selectedMaterial={selectedMaterial}
            style={{ marginLeft: `${entry.depth * 16}px` }}
          />
        </div>
      );
    },
    [flattenedTree, toggleNode, selectedMaterial],
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
  projectPath: string;
  selectMaterial: (materialPath: string) => void;
  selectedMaterial: string;
  style: React.CSSProperties;
};

const EntryRow: React.FC<EntryRowProps> = React.memo(
  ({
    entry,
    toggleNode,
    projectPath,
    selectMaterial,
    selectedMaterial,
    style,
  }) => {
    const entryPath = entry.path + (entry.isMat ? "/mat.yml" : "");
    const pathToEntry = buildPath(projectPath, entryPath);

    const handleClick = () => {
      if (!entry.isMat) {
        toggleNode(entry.path);
      } else {
        selectMaterial(pathToEntry);
      }
    };

    return (
      <div
        className={twMerge("my-1", entry.isMat && "border-l pl-0.5")}
        style={style}
      >
        <ContextMenu>
          <ContextMenuTrigger asChild>
            <Button
              size="sm"
              onClick={handleClick}
              variant={
                entry.isExpanded || pathToEntry === selectedMaterial
                  ? "secondary"
                  : "ghost"
              }
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
              <span
                className={
                  entry.isExpanded || pathToEntry === selectedMaterial
                    ? "font-medium"
                    : "font-normal"
                }
              >
                {entry.name}
              </span>
            </Button>
          </ContextMenuTrigger>
          <ContextMenuContent>
            <ContextMenuItem
              onClick={() => invoke("show_in_folder", { path: pathToEntry })}
            >
              <FolderOpen className="mr-2 size-5" />
              <span className="text-center text-sm">Show in folder</span>
            </ContextMenuItem>
          </ContextMenuContent>
        </ContextMenu>
      </div>
    );
  },
);
