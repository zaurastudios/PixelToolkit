export interface FileTree {
  name: string;
  is_mat?: true;
  children?: FileTree[];
}
