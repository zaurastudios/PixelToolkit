import * as Card from "@/components/ui/card";
import { Plus } from "lucide-react";
import { motion } from "framer-motion";
import * as Drawer from "@/components/ui/drawer";
import { Button } from "@/components/ui/button";
import { useState } from "react";

type CreateInitSelect = "new" | "existing";

export default function CreateProject() {
  const [initSelect, setInitSelect] = useState<CreateInitSelect>("new");

  const whileTap = { scale: 0.9 };
  const MotionCardComponent = motion(Card.Card);

  return (
    <Drawer.Drawer shouldScaleBackground>
      <Drawer.DrawerTrigger asChild>
        <MotionCardComponent
          whileTap={whileTap}
          className="cursor-pointer hover:bg-foreground/5 transition-colors"
        >
          <Card.CardHeader>
            <Card.CardTitle className="flex items-center gap-2 text-lg font-medium">
              <Plus height={20} /> Create new project
            </Card.CardTitle>
          </Card.CardHeader>
        </MotionCardComponent>
      </Drawer.DrawerTrigger>
      <Drawer.DrawerContent>
        <Drawer.DrawerHeader>
          <Drawer.DrawerTitle>Create a new project</Drawer.DrawerTitle>
        </Drawer.DrawerHeader>
        <Drawer.DrawerFooter>
          <Button>Submit</Button>
          <Drawer.DrawerClose />
        </Drawer.DrawerFooter>
      </Drawer.DrawerContent>
      <div />
    </Drawer.Drawer>
  );
}
