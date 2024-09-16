import {
  Menubar,
  MenubarContent,
  MenubarItem,
  MenubarMenu,
  MenubarSeparator,
  MenubarShortcut,
  MenubarTrigger,
} from "@/components/ui/menubar";
import { useKeyboardShortcut } from "@/lib/use-keyboard-shortcut";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useState } from "react";
import { Link, useLocation, useNavigate } from "react-router-dom";
import { Preferences } from "./preferences";

export const Navigation = () => {
  const [openPreferences, setOpenPreferences] = useState(false);

  const navigate = useNavigate();
  const location = useLocation();
  const pathname = location.pathname;
  const projectId = pathname.includes("project")
    ? pathname.split("/")[2]
    : null;

  async function closeWindow() {
    await getCurrentWindow().close();
  }

  // Navigate back to home
  useKeyboardShortcut({
    key: "H",
    handler: () => navigate("/"),
    ctrlKey: true,
  });

  // Navigate back to home
  useKeyboardShortcut({
    key: "Q",
    handler: closeWindow,
    ctrlKey: true,
  });

  // Open preferences
  const toggleOpenPreferences = () => setOpenPreferences((e) => !e);
  useKeyboardShortcut({
    key: ",",
    handler: toggleOpenPreferences,
    ctrlKey: true,
  });

  // Open in folder (project page)
  const showInFolder = () => {
    if (projectId) {
      invoke("show_in_folder", { path: projectId, isId: true });
    }
  };
  useKeyboardShortcut({
    key: "f",
    handler: showInFolder,
    ctrlKey: true,
    shiftKey: true,
  });

  return (
    <nav className="sticky top-0 h-7 w-full bg-zinc-200/80 dark:bg-zinc-900/50">
      <Menubar className="h-auto border-0 bg-transparent">
        <MenubarMenu>
          <MenubarTrigger className="px-1 py-0">App</MenubarTrigger>
          <MenubarContent>
            <MenubarItem asChild>
              <Link to="/">
                Home <MenubarShortcut className="font-mono">^H</MenubarShortcut>
              </Link>
            </MenubarItem>

            {pathname.includes("project") && (
              <MenubarItem onClick={showInFolder}>
                Show in folder{" "}
                <MenubarShortcut className="font-mono">^⇧F</MenubarShortcut>
              </MenubarItem>
            )}
            <MenubarSeparator />

            <MenubarItem onClick={toggleOpenPreferences}>
              Preferences{" "}
              <MenubarShortcut className="font-mono">^,</MenubarShortcut>
            </MenubarItem>
            <MenubarSeparator />

            <MenubarItem onClick={closeWindow}>
              Quit <MenubarShortcut className="font-mono">^Q</MenubarShortcut>
            </MenubarItem>
          </MenubarContent>
        </MenubarMenu>

        {pathname.includes("project") && (
          <>
            <MenubarMenu>
              <MenubarTrigger className="px-1 py-0">Project</MenubarTrigger>
              <MenubarContent></MenubarContent>
            </MenubarMenu>

            <MenubarMenu>
              <MenubarTrigger className="px-1 py-0">Pack</MenubarTrigger>
              <MenubarContent></MenubarContent>
            </MenubarMenu>

            <MenubarMenu>
              <MenubarTrigger className="px-1 py-0">Material</MenubarTrigger>
              <MenubarContent></MenubarContent>
            </MenubarMenu>
          </>
        )}
      </Menubar>
      <Preferences open={openPreferences} onOpenChange={setOpenPreferences} />
    </nav>
  );
};
