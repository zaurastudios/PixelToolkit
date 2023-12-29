import { FileTreeFolder } from "@/components/file-tree";
import { FileTreeProps } from "../../../main/ipc-events/project";

export default function Sidebar(props: { fileTree: FileTreeProps }) {
  const { fileTree } = props;

  return (
    <div className="">
      <FileTreeFolder fileTree={fileTree} />

      <div />
      <div />
    </div>
  );
}
