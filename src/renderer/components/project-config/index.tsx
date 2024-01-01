import React, { useEffect, useState } from "react";
import { motion } from "framer-motion";
import { Filter, Palette, Puzzle } from "lucide-react";
import * as Tooltip from "@/components/ui/tooltip";
import titleString from "@/utils/title";
import * as Select from "@/components/ui/select";

type TabOptions = "normal" | "ctm" | "filters";

const textureFilesOptions = [
  "General",
  "Color",
  "Opacity",
  "Height",
  "Normal",
  "Occlusion",
  "Smoothness",
  "Roughness",
  "Metal",
  "HCM",
  "F0",
  "Porosity",
  "SSS",
  "Emissive",
] as const;
type TextureFiles = (typeof textureFilesOptions)[number];

export default function ProjectConifg() {
  const [tabOption, setTabOption] = useState<TabOptions>("normal");
  const [textureFile, setTextureFile] = useState<TextureFiles>("Color");
  const [selectedTexture, setSelectedTexture] = useState<boolean>(true);

  window.electron.ipcRenderer.on("selected-texture", (arg) => {
    if (arg as string) {
      setSelectedTexture(false);
    } else {
      setSelectedTexture(true);
    }
  });

  useEffect(
    () =>
      window.electron.ipcRenderer.sendMessage(
        "select-texture-file",
        textureFile,
      ),
    [textureFile],
  );

  return (
    <div className="flex flex-col gap-4 p-2">
      <div className="grid grid-cols-3 gap-1 rounded-lg border p-1 shadow-sm">
        <Tooltip.TooltipProvider>
          {/* eslint-disable */}
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
          {/* eslint-enable */}
        </Tooltip.TooltipProvider>
      </div>

      <Select.Select onValueChange={(e) => setTextureFile(e as TextureFiles)}>
        <Select.SelectTrigger disabled={selectedTexture} className="w-full">
          <Select.SelectValue
            placeholder={textureFile}
            defaultValue={textureFile}
          />
        </Select.SelectTrigger>

        <Select.SelectContent>
          {textureFilesOptions.map((t) => (
            <Select.SelectItem key={t} value={t}>
              {t}
            </Select.SelectItem>
          ))}
        </Select.SelectContent>
      </Select.Select>
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
    <Tooltip.Tooltip>
      <Tooltip.TooltipTrigger asChild>
        <button
          className="group relative isolate inline-flex items-center justify-center whitespace-nowrap rounded-md p-2 text-sm font-medium text-foreground ring-offset-background transition-colors hover:bg-foreground/5 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50"
          type="button"
          onClick={() => setTabOption(value)}
        >
          {children}
          {selectedTab === value && (
            <motion.div
              className="absolute -z-[1] h-full w-full rounded-md bg-secondary text-secondary-foreground transition-colors group-hover:bg-secondary/80"
              layoutId="tab-option-config"
            />
          )}
        </button>
      </Tooltip.TooltipTrigger>
      <Tooltip.TooltipContent>
        {value === "ctm" ? "CTM" : titleString(value)}
      </Tooltip.TooltipContent>
    </Tooltip.Tooltip>
  );
}
