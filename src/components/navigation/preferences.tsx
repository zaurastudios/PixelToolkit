import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "../ui/dialog";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { getCurrentWindow } from "@tauri-apps/api/window";

export const Preferences = ({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: React.Dispatch<React.SetStateAction<boolean>>;
}) => {
  const storedTheme = localStorage.getItem("theme") as "light" | "dark" | null;
  let theme = storedTheme || "dark";

  const changeTheme = async (e: string) => {
    if (e === "system") {
      theme =
        ((await getCurrentWindow().theme()) as unknown as "dark" | "light") ??
        "dark";
    } else {
      theme = e as "dark" | "light";
    }

    document.documentElement.classList.remove("light", "dark");
    document.documentElement.classList.add(theme);

    localStorage.setItem("theme", theme);
  };

  return (
    <>
      <Dialog open={open} onOpenChange={onOpenChange}>
        <DialogContent className="max-w-3xl">
          <DialogHeader>
            <DialogTitle className="pb-4">Preferences</DialogTitle>
            <div className="rounded-lg border">
              <Tabs
                defaultValue="theme"
                className="grid grid-cols-[0.7fr_1.3fr] gap-4"
              >
                <TabsList className="flex h-auto flex-col justify-start rounded-r-none border-r bg-zinc-100 dark:bg-zinc-900/50">
                  <TabsTrigger value="theme" className="w-full">
                    Theme
                  </TabsTrigger>
                </TabsList>
                <TabsContent
                  value="theme"
                  className="m-0 p-4"
                  autoFocus={false}
                >
                  <DialogDescription>Set the app's theme</DialogDescription>
                  <Select onValueChange={changeTheme} defaultValue={theme}>
                    <SelectTrigger className="w-[180px]">
                      <SelectValue placeholder="Theme" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="light">Light</SelectItem>
                      <SelectItem value="dark">Dark</SelectItem>
                      <SelectItem value="system">System</SelectItem>
                    </SelectContent>
                  </Select>
                </TabsContent>
              </Tabs>
            </div>
          </DialogHeader>
        </DialogContent>
      </Dialog>
    </>
  );
};
