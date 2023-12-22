import * as Menu from "@/components/ui/menubar";
import { useTheme } from "next-themes";
import { Link, useNavigate } from "react-router-dom";

export default function MenuBar() {
  const navigate = useNavigate();
  const { theme, setTheme } = useTheme();

  function closeWindow() {
    window.electron.ipcRenderer.sendMessage("close-app");
  }

  // Route back to home page if native menu is reacted
  window.electron.ipcRenderer.on("go-home", () => navigate("/"));

  const changeTheme = (selectedTheme: "light" | "dark") =>
    setTheme(selectedTheme);
  console.log(theme);

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

        <Menu.MenubarMenu>
          <Menu.MenubarTrigger>Edit</Menu.MenubarTrigger>
        </Menu.MenubarMenu>

        <Menu.MenubarMenu>
          <Menu.MenubarTrigger>View</Menu.MenubarTrigger>

          {/* Quick theme switch */}
          <Menu.MenubarSeparator />
          <Menu.MenubarContent>
            <Menu.MenubarSub>
              <Menu.MenubarSubTrigger>Theme</Menu.MenubarSubTrigger>
              <Menu.MenubarSubContent>
                <Menu.MenubarRadioGroup
                  value={
                    theme === "dark"
                      ? "dark"
                      : theme === "light"
                        ? "light"
                        : "dark"
                  }
                >
                  <Menu.MenubarRadioItem
                    value="dark"
                    onClick={() => changeTheme("dark")}
                  >
                    Dark
                  </Menu.MenubarRadioItem>
                  <Menu.MenubarRadioItem
                    value="light"
                    onClick={() => changeTheme("light")}
                  >
                    Light
                  </Menu.MenubarRadioItem>
                </Menu.MenubarRadioGroup>
              </Menu.MenubarSubContent>
            </Menu.MenubarSub>
          </Menu.MenubarContent>
        </Menu.MenubarMenu>
      </Menu.Menubar>
    </nav>
  );
}
