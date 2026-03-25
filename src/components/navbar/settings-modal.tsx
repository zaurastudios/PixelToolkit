import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Moon, Sun } from "lucide-react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "../ui/dialog";
import { useTheme } from "../theme-provider";
import { titleString } from "@/lib/utils";

export function SettingsModal({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: React.Dispatch<React.SetStateAction<boolean>>;
}) {
  const { theme, setTheme } = useTheme();

  return (
    <>
      <Dialog open={open} onOpenChange={onOpenChange}>
        <DialogContent className="max-w-3xl">
          <DialogHeader>
            <DialogTitle className="pb-4">Settings</DialogTitle>
          </DialogHeader>
          <DialogDescription>
            <Tabs defaultValue="theme" orientation="vertical">
              <TabsList>
                <TabsTrigger value="theme">Theme</TabsTrigger>
              </TabsList>
              <TabsContent value="theme">
                <DropdownMenu>
                  <DropdownMenuTrigger
                    render={
                      <Button variant="outline" className="relative">
                        <Sun className="invisible" />
                        <Sun className="absolute left-2.5 h-[1.2rem] w-[1.2rem] scale-100 rotate-0 transition-all dark:scale-0 dark:-rotate-90" />
                        <Moon className="absolute left-2.5 h-[1.2rem] w-[1.2rem] scale-0 rotate-90 transition-all dark:scale-100 dark:rotate-0" />{" "}
                        {titleString(theme)}
                        <span className="sr-only">Toggle theme</span>
                      </Button>
                    }
                  />
                  <DropdownMenuContent align="end">
                    <DropdownMenuItem onClick={() => setTheme("light")}>
                      Light
                    </DropdownMenuItem>
                    <DropdownMenuItem onClick={() => setTheme("dark")}>
                      Dark
                    </DropdownMenuItem>
                    <DropdownMenuItem onClick={() => setTheme("system")}>
                      System
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </TabsContent>
            </Tabs>
          </DialogDescription>
        </DialogContent>
      </Dialog>
    </>
  );
}
