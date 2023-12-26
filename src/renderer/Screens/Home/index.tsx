import * as Card from "@/components/ui/card";
import Projects from "./projects";

export default function Home() {
  window.electron.ipcRenderer.sendMessage("set-title", "PixelToolkit");

  return (
    <main>
      <section className="p-4">
        <Card.Card className="rounded-xl">
          <Card.CardHeader>
            <Card.CardTitle className="text-4xl font-semibold">
              PixelToolkit
            </Card.CardTitle>
            <Card.CardDescription className="text-base">
              An alternative to{" "}
              <a
                href="https://github.com/null511/PixelGraph-Release"
                target="_blank"
                rel="noreferrer"
                className="underline text-foreground/90"
              >
                PixelGraph
              </a>
              . It allows you to convert raw texture files, or{" "}
              <code className="bg-foreground/10 rounded-lg px-2 py-1">
                .sbs
              </code>{" "}
              files to a PBR format that is accepted in Minecraft.
            </Card.CardDescription>
          </Card.CardHeader>
        </Card.Card>
      </section>
      <section className="p-4 pt-0">
        <Projects />
      </section>
    </main>
  );
}
