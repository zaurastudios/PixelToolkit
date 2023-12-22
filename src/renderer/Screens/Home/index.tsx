import * as Card from "@/components/ui/card";
import { Link } from "react-router-dom";

export default function Home() {
  return (
    <main>
      <section className="p-4">
        <Card.Card>
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
                className="underline text-white/90"
              >
                PixelGraph
              </a>
              . It allows you to work in a &quot;raw&quot; texture space and
              automates publishing to one or more encodings, rather than trying
              to directly encode your textures as design-time. Yaml
              configuration files can also be used to apply final adjustments to
              your compiled textures. A cross-platform command-line version is
              also available, allowing you to completely automating your
              publishing process from your remote content repository.
            </Card.CardDescription>
          </Card.CardHeader>
        </Card.Card>
      </section>
      <h1>Hii</h1>
      <Link to="/test" className="underliner">
        Go to test
      </Link>
    </main>
  );
}
