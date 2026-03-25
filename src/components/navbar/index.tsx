import {
  Menubar,
  MenubarContent,
  MenubarGroup,
  MenubarItem,
  MenubarMenu,
  MenubarSeparator,
  MenubarShortcut,
  MenubarTrigger,
} from "@/components/ui/menubar";
import { useKeyboardShortcut } from "@/lib/use-kb-shortcut";
import { Link, useLocation, useNavigate } from "@tanstack/react-router";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useState } from "react";
import { ShortcutsKbd } from "../shortcuts";
import { SettingsModal } from "./settings-modal";

export function Navbar() {
  const [openSettings, setOpenSettings] = useState(false);

  const location = useLocation();
  const navigate = useNavigate();

  const projectId = location.href.includes("project") ? location.href : null;

  async function closeWindow() {
    await getCurrentWindow().close();
  }

  useKeyboardShortcut({
    key: "H",
    handler: () => navigate({ to: "/" }),
    ctrlKey: true,
  });
  useKeyboardShortcut({
    key: "Q",
    handler: closeWindow,
    ctrlKey: true,
  });

  const toggleSettings = () => setOpenSettings((e) => !e);
  useKeyboardShortcut({
    key: ",",
    handler: toggleSettings,
    ctrlKey: true,
  });

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
    <>
      <nav>
        <Menubar className="border-0 border-b">
          <MenubarMenu>
            <MenubarTrigger>File</MenubarTrigger>
            <MenubarContent>
              <MenubarGroup>
                <MenubarItem
                  render={
                    <Link to="/">
                      Home{" "}
                      <MenubarShortcut>
                        <ShortcutsKbd ctrlKey shortcut="H" />
                      </MenubarShortcut>
                    </Link>
                  }
                />

                <MenubarItem onClick={toggleSettings}>
                  Settings{" "}
                  <MenubarShortcut>
                    <ShortcutsKbd ctrlKey shortcut="," />
                  </MenubarShortcut>
                </MenubarItem>

                <MenubarSeparator />

                <MenubarItem onClick={closeWindow}>
                  Quit{" "}
                  <MenubarShortcut>
                    <ShortcutsKbd ctrlKey shortcut="Q" />
                  </MenubarShortcut>
                </MenubarItem>
              </MenubarGroup>
            </MenubarContent>
          </MenubarMenu>

          {projectId && (
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
      </nav>

      <SettingsModal open={openSettings} onOpenChange={setOpenSettings} />
    </>
  );
}
