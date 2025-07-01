import { readdirSync } from "fs";
import { join } from "path";
import type { SidebarConfig } from "vuepress-vite";

const structPath = join(__dirname, "..", "..", "docs");
const allFiles = readdirSync(structPath)
    .filter(file => file.endsWith(".md"))
    .map(file => `/docs/${file}`);

const excludedFiles = [
    "/docs/getting-started.md",
    "/docs/FAQ.md"
];

export const sidebar: SidebarConfig = [
    {
        text: "",
        children: excludedFiles.filter(file => allFiles.includes(file))
    },
];