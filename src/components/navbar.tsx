import {
  Menubar,
  MenubarCheckboxItem,
  MenubarContent,
  MenubarGroup,
  MenubarItem,
  MenubarMenu,
  MenubarRadioGroup,
  MenubarRadioItem,
  MenubarSeparator,
  MenubarShortcut,
  MenubarSub,
  MenubarSubContent,
  MenubarSubTrigger,
  MenubarTrigger,
} from "@/components/ui/menubar";
import { useKeyboardShortcut } from "@/lib/use-kb-shortcut";
import { Link, useLocation, useNavigate } from "@tanstack/react-router";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { ShortcutsKbd } from "./shortcuts";
import { invoke } from "@tauri-apps/api/core";

export function Navbar() {
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
    <nav>
      <Menubar className="border-x-0">
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
  );
}
