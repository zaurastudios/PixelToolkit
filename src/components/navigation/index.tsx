import { Link, useNavigate } from "react-router-dom";
import {
  Menubar,
  MenubarMenu,
  MenubarItem,
  MenubarTrigger,
  MenubarShortcut,
  MenubarContent,
  MenubarSeparator,
} from "@/components/ui/menubar";
import { useKeyboardShortcut } from "@/lib/use-keyboard-shortcut";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Preferences } from "./preferences";

export const Navigation = () => {
  const navigate = useNavigate();

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

  return (
    <header className="p-1">
      <nav className="w-full rounded-lg bg-zinc-200/80 dark:bg-zinc-900/50">
        <Menubar className="h-auto border-0 bg-transparent">
          <MenubarMenu>
            <MenubarTrigger className="px-1 py-0">File</MenubarTrigger>
            <MenubarContent>
              <MenubarItem asChild>
                <Link to="/">
                  Home{" "}
                  <MenubarShortcut className="font-mono">^H</MenubarShortcut>
                </Link>
              </MenubarItem>

              <MenubarItem onClick={closeWindow}>
                Quit <MenubarShortcut className="font-mono">^Q</MenubarShortcut>
              </MenubarItem>
            </MenubarContent>
          </MenubarMenu>

          <MenubarMenu>
            <MenubarTrigger className="px-1 py-0">Edit</MenubarTrigger>
            <MenubarContent>
              <MenubarSeparator />
              <Preferences />
            </MenubarContent>
          </MenubarMenu>
        </Menubar>
      </nav>
    </header>
  );
};
