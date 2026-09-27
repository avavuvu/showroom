import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { fileURLToPath } from "node:url";
import { existsSync } from "node:fs";

const localBoutique = fileURLToPath(new URL("../../../dev/boutique/bq_components/src", import.meta.url));
const boutique = existsSync(localBoutique)
    ? localBoutique
    : fileURLToPath(new URL("./node_modules/boutique/bq_components/src", import.meta.url));

export default defineConfig({
    publicDir: false,
    base: "/build/",
    plugins: [vue()],
    resolve: {
        alias: {
            "@bq": boutique,
        },
    },
    build: {
        outDir: "public/build",
        emptyOutDir: true,
        manifest: true,
        rollupOptions: {
            input: {
                site: "resources/js/site.ts",
                ascii: "resources/js/ascii/index.ts",
                islands: "resources/js/islands.ts",
            },
            output: {
                entryFileNames: "[name]-[hash].js",
                chunkFileNames: "[name]-[hash].js",
                assetFileNames: "[name]-[hash].[ext]",
            },
        },
    },
});
