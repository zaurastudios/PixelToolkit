import React, { useEffect, useState } from "react";
import { Filter, Palette, Puzzle } from "lucide-react";
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipProvider,
} from "@/components/ui/tooltip";
import { titleString } from "@/lib/utils";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { listen } from "@tauri-apps/api/event";

import { DefaultTab } from "./tabs";
import { textureFilesOptions, TextureFilesTypes } from "@/types/interface";

type TabOptions = "normal" | "ctm" | "filters";

export default function ProjectConfig({
  texturePath,
}: {
  texturePath: string;
}) {
  const [tabOption, setTabOption] = useState<TabOptions>("normal");
  const [textureFile, setTextureFile] = useState<TextureFilesTypes>("Normal");
  const [materialPath, setMaterialPath] = useState("");

  useEffect(() => {
    async function init() {
      const unlisten = await listen<string>("selected-texture", (e) =>
        setMaterialPath(e.payload),
      );

      return () => unlisten();
    }

    init();
  }, [texturePath]);

  return (
    <div className="flex flex-col gap-4 p-2">
      <div className="grid grid-cols-3 gap-1 rounded-lg border p-1 shadow-sm">
        <TooltipProvider>
          <TabButton
            value="normal"
            selectedTab={tabOption}
            setTabOption={setTabOption}
          >
            <Palette className="size-5" />
          </TabButton>
          <TabButton
            value="ctm"
            selectedTab={tabOption}
            setTabOption={setTabOption}
          >
            <Puzzle className="size-5" />
          </TabButton>
          <TabButton
            value="filters"
            selectedTab={tabOption}
            setTabOption={setTabOption}
          >
            <Filter className="size-5" />
          </TabButton>
        </TooltipProvider>
      </div>

      <Select
        onValueChange={(e: any) => setTextureFile(e as TextureFilesTypes)}
      >
        <SelectTrigger disabled={!materialPath} className="w-full">
          <SelectValue placeholder={textureFile} defaultValue={textureFile} />
        </SelectTrigger>

        <SelectContent className="max-h-none">
          {textureFilesOptions.map((t) => (
            <SelectItem key={t} value={t}>
              {t}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>

      {!!materialPath && tabOption === "normal" && (
        <DefaultTab
          textureFileOption={textureFile}
          materialPath={materialPath}
        />
      )}
    </div>
  );
}

interface TabButtonProps {
  value: TabOptions;
  children: React.ReactNode;
  selectedTab: TabOptions;
  setTabOption: React.Dispatch<React.SetStateAction<TabOptions>>;
}
function TabButton({
  value,
  children,
  selectedTab,
  setTabOption,
}: TabButtonProps) {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <button
          className="group relative isolate inline-flex items-center justify-center whitespace-nowrap rounded-md p-2 text-sm font-medium text-foreground ring-offset-background transition-colors hover:bg-foreground/5 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50"
          type="button"
          onClick={() => setTabOption(value)}
        >
          {children}
          {selectedTab === value && (
            <div className="absolute -z-[1] h-full w-full rounded-md bg-secondary text-secondary-foreground transition-colors group-hover:bg-secondary/80" />
          )}
        </button>
      </TooltipTrigger>
      <TooltipContent>
        {value === "ctm" ? "CTM" : titleString(value)}
      </TooltipContent>
    </Tooltip>
  );
}
