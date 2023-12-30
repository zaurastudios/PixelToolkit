import { FileTreeFolder } from "@/components/file-tree";
import { FileTreeProps } from "../../../main/ipc-events/project";

export default function Sidebar(props: { fileTree: FileTreeProps }) {
  const { fileTree } = props;

  const [query, setQuery] = useState("");

  function filterTree(tree: FileTreeProps): FileTreeProps | undefined {
    const filter =
      tree.children &&
      tree.children.length > 0 &&
      (tree.children
        .map((child) => filterTree(child))
        .filter((child) => child !== undefined) as FileTreeProps[]);

    const compareQueryAndName =
      tree.name.toLowerCase().includes(query.toLowerCase()) ||
      query.toLowerCase().includes(tree.name.toLowerCase());
    if (tree.isMat && compareQueryAndName) {
      return tree;
    }

    if (!tree.isMat && tree.name !== query && filter && filter.length > 0) {
      return { name: tree.name, children: filter };
    }

    return undefined;
  }

  return (
    <div className="">
      <FileTreeFolder fileTree={fileTree} />

      <div />
      <div />
    </div>
  );
}
