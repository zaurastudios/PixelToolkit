export default function titleString(str: string): string {
  return str
    .split(" ")
    .map((s) => s[0].charAt(0).toUpperCase() + s.slice(1))
    .join(" ");
}
