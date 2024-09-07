import { FileTree } from "@/types/project";

export function filterTree(
  tree: FileTree,
  query: string,
): FileTree | undefined {
  const filter =
    tree.children &&
    tree.children.length > 0 &&
    (tree.children
      .map((child) => filterTree(child, query))
      .filter((child) => child !== undefined) as FileTree[]);

  const compareQueryAndName =
    tree.name.toLowerCase().includes(query.toLowerCase()) ||
    query.toLowerCase().includes(tree.name.toLowerCase());
  if (tree.is_mat && compareQueryAndName) {
    return tree;
  }

  if (!tree.is_mat && tree.name !== query && filter && filter.length > 0) {
    return { name: tree.name, is_mat: false, children: filter };
  }

  return undefined;
}
