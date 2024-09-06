import { listen } from "@tauri-apps/api/event";
import { toast } from "sonner";

export async function simpleToast() {
  await listen<string>("simple-toast", (e) =>
    toast(e.payload, {
      duration: 1.5,
    }),
  );
}
