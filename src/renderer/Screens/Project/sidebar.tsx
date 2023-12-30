import { FileTreeFolder } from "@/components/file-tree";
import filterTree from "@/utils/filter-tree";
import React, { useState } from "react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Search } from "lucide-react";
import { FileTreeProps } from "../../../main/ipc-events/project";

export default function Sidebar(props: { fileTree: FileTreeProps }) {
  const { fileTree } = props;

  const [query, setQuery] = useState<string>("");

  function modifiedFileTree(): FileTreeProps {
    if (query) {
      const filteredTree = filterTree(fileTree, query);
      if (filteredTree) {
        return filteredTree;
      }
    }

    return fileTree;
  }

  function submitHandler(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();

    const formData = new FormData(e.currentTarget);
    const searchQuery = formData.get("search")! as string;
    setQuery(searchQuery);
  }

  return (
    <>
      <form
        className="sticky bg-background z-20 rounded-b-xl shadow-sm w-full left-0 top-0 p-2 flex"
        onSubmit={submitHandler}
      >
        <div className="flex w-full items-center gap-2">
          <Input
            type="text"
            placeholder="Search..."
            name="search"
            className="grow max-w-none min-w-none w-auto"
          />
          <Button size="icon">
            <Search className="size-4" />
          </Button>
        </div>
      </form>

      <FileTreeFolder fileTree={modifiedFileTree()} query={query} />

      <div />
      <div />
    </>
  );
}
