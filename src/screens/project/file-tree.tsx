import { FileTreeFolder } from "@/components/file-tree-buttons";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { filterTree } from "@/lib/filter-trees";
import { FileTree } from "@/types/project";
import { TooltipPortal } from "@radix-ui/react-tooltip";
import { FolderSync } from "lucide-react";
import { useEffect, useState } from "react";
import { useDebounce } from "react-use";

export const FileTreeSidebar: React.FC<{
  fileTree: FileTree;
  projectPath: string;
  updateFileTree: () => Promise<void>;
}> = ({ fileTree, projectPath, updateFileTree }) => {
  const [filteredFileTree, setFilteredFileTree] = useState(fileTree);
  const [query, setQuery] = useState<string>("");

  useEffect(() => {
    setFilteredFileTree(fileTree);
  }, [fileTree]);

  useDebounce(
    () => {
      if (query) {
        const filteredTree = filterTree(fileTree, query);
        setFilteredFileTree(filteredTree || fileTree);
      } else {
        setFilteredFileTree(fileTree);
      }
    },
    250,
    [query, fileTree],
  );

  const handleUpdateFileTree = async () => {
    await updateFileTree();
  };

  return (
    <>
      <div className="sticky top-0 z-10 flex gap-1 border-b bg-background p-1">
        <Input
          type="text"
          placeholder="Search..."
          name="search"
          value={query}
          onChange={(e) => setQuery(e.currentTarget.value)}
          className="min-w-none z-10 w-auto max-w-none grow"
        />
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant="outline"
                size="icon"
                onClick={handleUpdateFileTree}
              >
                <FolderSync className="size-4" />
              </Button>
            </TooltipTrigger>
            <TooltipPortal>
              <TooltipContent>Resync Folder</TooltipContent>
            </TooltipPortal>
          </Tooltip>
        </TooltipProvider>
      </div>
      <FileTreeFolder
        fileTree={filteredFileTree}
        query={query}
        projectPath={projectPath}
      />
    </>
  );
};
