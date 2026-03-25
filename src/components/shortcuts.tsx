import { platform } from "@tauri-apps/plugin-os";
import { Kbd, KbdGroup } from "./ui/kbd";

interface Props {
  ctrlKey?: boolean;
  shiftKey?: boolean;
  altKey?: boolean;
  shortcut: string | string[];
}

export function ShortcutsKbd({ ctrlKey, shiftKey, altKey, shortcut }: Props) {
  const currentPlatform = platform();

  return (
    <KbdGroup>
      {ctrlKey && <Kbd>{currentPlatform === "macos" ? "⌘" : "^"}</Kbd>}
      {altKey && <Kbd>⌥</Kbd>}
      {shiftKey && <Kbd>⇧</Kbd>}
      {typeof shortcut === "string" ? (
        <Kbd>{shortcut}</Kbd>
      ) : (
        shortcut.map((s, i) => <Kbd key={i}>{s}</Kbd>)
      )}
    </KbdGroup>
  );
}
