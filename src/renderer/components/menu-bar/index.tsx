import * as Menu from "@/components/ui/menubar";
import { Link, useNavigate } from "react-router-dom";

export default function MenuBar() {
  const navigate = useNavigate();
  function closeWindow() {
    window.electron.ipcRenderer.sendMessage("close-app");
  }

  // Route back to home page if native menu is reacted
  window.electron.ipcRenderer.on("go-home", () => navigate("/"));

  return (
    <nav className="p-4 w-max pb-0">
      <Menu.Menubar>
        <Menu.MenubarMenu>
          <Menu.MenubarTrigger>File</Menu.MenubarTrigger>
          <Menu.MenubarContent>
            <Menu.MenubarItem asChild>
              <Link to="/">
                Home <Menu.MenubarShortcut>⌃H</Menu.MenubarShortcut>
              </Link>
            </Menu.MenubarItem>
            <Menu.MenubarItem onClick={() => closeWindow()}>
              Quit <Menu.MenubarShortcut>⌃Q</Menu.MenubarShortcut>
            </Menu.MenubarItem>
          </Menu.MenubarContent>
        </Menu.MenubarMenu>
      </Menu.Menubar>
    </nav>
  );
}
