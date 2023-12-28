import { AnimatePresence, motion } from "framer-motion";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";

export default function NewProjectContent() {
  return (
    <AnimatePresence>
      <motion.div
        key="show-once-path-is-selected"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        className="flex flex-col mt-4 gap-4"
      >
        <div className="flex gap-4">
          <div className="grid w-full max-w-sm items-center gap-1.5">
            <Label aria-required htmlFor="project">
              Project Name{" "}
              <span className="text-red-500" aria-hidden>
                *
              </span>
            </Label>
            <Input
              type="text"
              name="project"
              placeholder="My Amazing Project"
              required
            />
          </div>
          <div className="grid w-full max-w-sm items-center gap-1.5">
            <Label aria-required htmlFor="description">
              Project Description{" "}
            </Label>
            <Input
              type="text"
              name="description"
              placeholder="My amazing project description"
            />
          </div>
        </div>
        <Button className="w-max" type="submit">
          Create
        </Button>
      </motion.div>
    </AnimatePresence>
  );
}
