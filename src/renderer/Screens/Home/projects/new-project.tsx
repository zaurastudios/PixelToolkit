import { AnimatePresence, motion } from "framer-motion";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useState } from "react";
import { Button } from "@/components/ui/button";

export default function NewProjectContent() {
  const [title, setTitle] = useState("");

  return (
    <AnimatePresence>
      <motion.div
        key="show-once-path-is-selected"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        className="flex flex-col gap-4"
      >
        <div className="grid w-full max-w-sm items-center gap-1.5">
          <Label aria-required htmlFor="project">
            Project Name{" "}
            <span className="text-red-500" aria-hidden>
              *
            </span>
          </Label>
          <Input
            type="text"
            id="project"
            name="project"
            placeholder="My Amazing Project"
            required
            onChange={(e) => setTitle(e.target.value)}
          />
        </div>

        {title && <Button className="w-max">Create</Button>}
      </motion.div>
    </AnimatePresence>
  );
}
