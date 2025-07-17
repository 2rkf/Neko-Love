import { defaultTheme, defineUserConfig } from "vuepress-vite";
import { searchPlugin } from "@vuepress/plugin-search";
import { navbar } from "./configs/navbar";
import { sidebar } from "./configs/sidebar";

export default defineUserConfig({
    lang: "en-GB",
    base: "/",
    title: "Nekoi",
    head: [
        ["link", { rel: "icon", href: "/favicon.ico" }],
        ["meta", { property: "og:title", content: "Nekoi" }],
        ["meta", { property: "og:image", content: "/hero.png" }],
        ["meta", { property: "og:description", content: "Nekoi is a collection of high-quality images featuring characters with anime-style appearances." }],
        ["meta", { property: "theme-color", content: "#ff7b25" }],
    ],
    description: "Nekoi is a collection of high-quality images featuring characters with anime-style appearances.",
    extendsPage: (data) => {
        if (data.frontmatter.permalink) {
            return;
        }

        if (data.path.endsWith(".html")) {
            data.path = data.path.slice(0, -5);
        } else if (data.dataFilePath.endsWith("/")) {
            data.path = data.path.slice(0, -1);
        }
    },
    pagePatterns: ["**/*.md", "!**/README.md", "!.vuepress", "!node_modules"],
    /* @ts-ignore */
    plugins: [searchPlugin({ locales: { "/": { placeholder: "Search" } } })],
    theme: defaultTheme({
        navbar,
        home: "/",
        sidebar,
        contributors: false,
        lastUpdated: false,
        toggleColorMode: "Switch Theme",
        sidebarDepth: 3,
        docsDir: "docs",
        editLink: false,
    })
});
