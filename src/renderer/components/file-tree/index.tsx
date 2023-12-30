import { useState } from "react";
import { ChevronRight, Paintbrush } from "lucide-react";
import { v4 as uuidv4 } from "uuid";
import similarity from "@/utils/similarity";
import { FileTreeProps } from "../../../main/ipc-events/project";
import { Button } from "../ui/button";

export function FileTreeFolder(props: { fileTree: FileTreeProps }) {
  const { fileTree } = props;

  return (
    <div className="p-2">
      {fileTree.children?.map((entry) => (
        // eslint-disable-next-line
        <Entry key={uuidv4()} entry={entry} depth={1} />
      ))}
    </div>
  );
}

// Article referred to: https://blog.stackademic.com/creating-a-folder-tree-view-component-with-react-and-typescript-d3df2086878e
type EntryProps = {
  entry: FileTreeProps;
  depth: number;
};

export function Entry({ entry, depth }: EntryProps) {
  const [isExpanded, setIsExpanded] = useState<boolean>(false);

  return (
    <>
      <Button
        onClick={() => {
          if (!entry.isMat) setIsExpanded((prev) => !prev);
        }}
        variant={isExpanded ? "secondary" : "ghost"}
        className="py-2 leading-3 h-max px-2 pr-3"
      >
        {entry.isMat && <Paintbrush className="size-4 mr-2 opacity-50" />}
        {!entry.isMat && entry.children && (
          <div>
            <ChevronRight
              className={`size-4 mr-2 opacity-50 ${
                isExpanded ? "rotate-90" : undefined
              }`}
            />
          </div>
        )}
        <span>{entry.name}</span>
      </Button>
      <div className="border-l ml-1 mt-1">
        {isExpanded && (
          <div className="ml-1">
            {" "}
            {entry.children?.map((e) => (
              <Entry key={uuidv4()} entry={e} depth={depth + 1} />
            ))}
          </div>
        )}
      </div>
    </>
  );
}
