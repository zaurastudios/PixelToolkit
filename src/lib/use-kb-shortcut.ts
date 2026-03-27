import { useEffect, useCallback } from "react";
import { platform } from "@tauri-apps/plugin-os";

type KeyHandler = (event: KeyboardEvent) => void;

interface UseKeyboardShortcutProps {
  key: string;
  handler: KeyHandler;
  eventType?: "keydown" | "keyup";
  ctrlKey?: boolean;
  shiftKey?: boolean;
  altKey?: boolean;
  metaKey?: boolean;
}

const isMac = () => platform() === "macos";

export const useKeyboardShortcut = ({
  key,
  handler,
  eventType = "keydown",
  ctrlKey = false,
  shiftKey = false,
  altKey = false,
  metaKey = false,
}: UseKeyboardShortcutProps): void => {
  const handleKeyEvent = useCallback(
    (event: KeyboardEvent) => {
      const requiresMeta = isMac() ? ctrlKey || metaKey : metaKey;
      const requiresCtrl = isMac() ? false : ctrlKey;

      const ctrlMatch = requiresCtrl
        ? event.ctrlKey
        : !event.ctrlKey || isMac();
      const metaMatch = requiresMeta ? event.metaKey : !event.metaKey;
      const shiftMatch = shiftKey ? event.shiftKey : !event.shiftKey;
      const altMatch = altKey ? event.altKey : !event.altKey;
      const keyMatch = event.key.toLowerCase() === key.toLowerCase();

      if (keyMatch && ctrlMatch && metaMatch && shiftMatch && altMatch) {
        event.preventDefault();
        event.stopPropagation();
        handler(event);
      }
    },
    [key, handler, ctrlKey, shiftKey, altKey, metaKey],
  );

  useEffect(() => {
    window.addEventListener(eventType, handleKeyEvent, { capture: true });

    return () => {
      window.removeEventListener(eventType, handleKeyEvent, { capture: true });
    };
  }, [eventType, handleKeyEvent]);
};
