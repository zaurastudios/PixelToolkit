import { useEffect } from "react";
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

export const useKeyboardShortcut = ({
  key,
  handler,
  eventType = "keydown",
  ctrlKey = false,
  shiftKey = false,
  altKey = false,
  metaKey = false,
}: UseKeyboardShortcutProps) => {
  useEffect(() => {
    const handleKeyEvent = (event: KeyboardEvent) => {
      if (
        event.key.toLowerCase() === key.toLowerCase() &&
        (platform() === "macos"
          ? event.metaKey === ctrlKey
          : event.ctrlKey === ctrlKey) &&
        event.shiftKey === shiftKey &&
        event.altKey === altKey &&
        event.metaKey === metaKey
      ) {
        handler(event);
      }
    };

    window.addEventListener(eventType, handleKeyEvent);

    return () => {
      window.removeEventListener(eventType, handleKeyEvent);
    };
  }, [key, handler, eventType, ctrlKey, shiftKey, altKey, metaKey]);
};
