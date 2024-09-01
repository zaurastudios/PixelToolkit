import { MenubarItem, MenubarShortcut } from "../ui/menubar";

export const Preferences = () => {
  return (
    <>
      <MenubarItem>
        Preferences <MenubarShortcut className="font-mono">^,</MenubarShortcut>
      </MenubarItem>
    </>
  );
};
