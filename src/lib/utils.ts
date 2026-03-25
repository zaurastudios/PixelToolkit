import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function titleString(str: string): string {
  return str
    .split(" ")
    .map((s) => s[0].charAt(0).toUpperCase() + s.slice(1))
    .join(" ");
}
