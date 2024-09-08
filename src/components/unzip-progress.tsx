import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { FixedSizeList as List } from "react-window";

export function UnzipProgress() {
  const [open, setOpen] = useState(false);
  const [curr, setCurr] = useState<string[]>([]);

  const [startTime, setStartTime] = useState(new Date());
  const [endTime, setEndTime] = useState(new Date());

  listen<boolean>("unzip-started", (e) => setOpen(e.payload));

  async function init() {
    const unlisten = await listen<string>("unzip-progress", (e) => {
      setCurr((prev) => [e.payload, ...prev]);
      if (!e.payload) {
        setCurr([]);
        setEndTime(new Date());
      } else {
        setStartTime(new Date());
      }
    });

    return () => unlisten();
  }

  useEffect(() => {
    let timeDiff = +endTime - +startTime;
    console.log(endTime, startTime);
    console.log(Math.round(timeDiff));
    timeDiff /= 1000;
    console.log(Math.round(timeDiff));
  }, [endTime]);

  useEffect(() => {
    init();
  }, []);

  const Row = ({
    index,
    style,
  }: {
    index: number;
    style: React.CSSProperties;
  }) => {
    const className = curr[index].length > 107 ? "text-xs" : "text-sm";

    return (
      <span style={style} className={className}>
        {curr[index]}
      </span>
    );
  };

  return (
    <Dialog open={open}>
      <DialogContent showCloseButton={false}>
        <DialogHeader>
          <DialogTitle className="pb-2">Progress</DialogTitle>

          <code className="break-all rounded-md border p-2 text-sm">
            <List
              height={200}
              itemCount={curr.length}
              itemSize={40}
              layout="vertical"
              width={444}
            >
              {Row}
            </List>
          </code>
        </DialogHeader>
      </DialogContent>
    </Dialog>
  );
}
