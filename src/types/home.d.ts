export interface Project {
  id: string;
  path: string;
  name: string;
  description?: string;
  pack_image?: string;
  date_modified: string;
}

export interface ProjectInfo {
  name: string | null;
  description?: string | null;
}
