import { FileTreeFolder } from "@/components/file-tree-buttons";
import { filterTree } from "@/lib/filter-trees";
import { FileTree } from "@/types/project";
import { useEffect, useState } from "react";
import { useDebounce } from "react-use";
import {} from "@/lib/filter-trees";
import { Input } from "@/components/ui/input";

export const FileTreeSidebar = ({
  fileTree,
  projectPath,
}: {
  fileTree: FileTree;
  projectPath: string;
}) => {
  const [filteredFileTree, setFilteredFileTree] = useState(fileTree);
  const [query, setQuery] = useState<string>("");

  const [_, cancel] = useDebounce(
    () => {
      if (query) {
        const filteredTree = filterTree(filteredFileTree, query);
        if (filteredTree) {
          setFilteredFileTree(filteredTree);
        } else {
          setFilteredFileTree(fileTree);
        }
      } else {
        setFilteredFileTree(fileTree);
      }
    },
    0,
    [query],
  );

  return (
    <>
      <div className="sticky top-0 z-10 flex gap-2 border-b bg-background p-1">
        <Input
          type="text"
          placeholder="Search..."
          name="search"
          value={query}
          onChange={(e) => setQuery(e.currentTarget.value)}
          className="min-w-none z-10 w-auto max-w-none grow"
        />
      </div>

      <FileTreeFolder
        fileTree={filteredFileTree}
        query={query}
        projectPath={projectPath}
      />
    </>
  );
};
