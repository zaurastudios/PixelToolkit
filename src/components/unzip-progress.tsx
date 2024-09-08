import React, { useEffect, useState, useCallback, useMemo } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { listen } from "@tauri-apps/api/event";
import { FixedSizeList as List } from "react-window";

export function UnzipProgress() {
  const [open, setOpen] = useState(false);
  const [curr, setCurr] = useState<string[]>([]);

  useEffect(() => {
    const unzipStartedUnlisten = listen<boolean>("unzip-started", (e) =>
      setOpen(e.payload),
    );
    return () => {
      unzipStartedUnlisten.then((unlisten) => unlisten());
    };
  }, []);

  useEffect(() => {
    let updateTimeout: NodeJS.Timeout | null;
    const updates: string[] = [];

    async function init() {
      const unlisten = await listen<string>("unzip-progress", (e) => {
        updates.push(e.payload);

        if (!updateTimeout) {
          updateTimeout = setTimeout(() => {
            setCurr((prev) => [...updates, ...prev].slice(0, 1000));
            updates.length = 0;
            updateTimeout = null;
          }, 10);
        }

        if (!e.payload) {
          setCurr([]);
        }
      });

      return () => {
        unlisten();
        if (updateTimeout) {
          clearTimeout(updateTimeout);
        }
      };
    }

    init();
  }, []);

  const Row = useCallback(
    ({ index, style }: { index: number; style: React.CSSProperties }) => {
      const item = curr[index];
      const className = item.length > 107 ? "text-xs" : "text-sm";
      return (
        <span style={style} className={className}>
          {item}
        </span>
      );
    },
    [curr],
  );

  const memoizedList = useMemo(
    () => (
      <List
        height={200}
        itemCount={curr.length}
        itemSize={40}
        layout="vertical"
        width={444}
      >
        {Row}
      </List>
    ),
    [curr, Row],
  );

  return (
    <Dialog open={open}>
      <DialogContent showCloseButton={false}>
        <DialogHeader>
          <DialogTitle className="pb-2">Progress</DialogTitle>
          <code className="break-all rounded-md border p-2 text-sm">
            {memoizedList}
          </code>
        </DialogHeader>
      </DialogContent>
    </Dialog>
  );
}
