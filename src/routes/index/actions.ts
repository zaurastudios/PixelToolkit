import { Project } from "@/types/home";
import { invoke } from "@tauri-apps/api/core";

export async function getProjects(): Promise<Project[]> {
  return invoke<Project[]>("get_projects");
}
