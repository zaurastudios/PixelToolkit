import { useEffect } from "react";

type KeyHandler = (event: KeyboardEvent) => void;

interface UseKeyboardShortcutOptions {
  key: string; // The key that should trigger the handler
  handler: KeyHandler; // The handler function to be called
  eventType?: "keydown" | "keyup"; // The event type to listen for (default is 'keydown')
  ctrlKey?: boolean; // Optional modifier keys
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
}: UseKeyboardShortcutOptions) => {
  useEffect(() => {
    const handleKeyEvent = (event: KeyboardEvent) => {
      if (
        event.key.toLowerCase() === key.toLowerCase() &&
        event.ctrlKey === ctrlKey &&
        event.shiftKey === shiftKey &&
        event.altKey === altKey &&
        event.metaKey === metaKey
      ) {
        handler(event);
      }
    };

    // Add the event listener
    window.addEventListener(eventType, handleKeyEvent);

    // Remove the event listener on cleanup
    return () => {
      window.removeEventListener(eventType, handleKeyEvent);
    };
  }, [key, handler, eventType, ctrlKey, shiftKey, altKey, metaKey]);
};
