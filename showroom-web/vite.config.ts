import { defineConfig } from "vite";
import { fileURLToPath } from "node:url";

export default defineConfig({
    publicDir: false,
    base: "/build/",

    resolve: {
        alias: {
            "@setups": fileURLToPath(new URL("./bindings/setups.ts", import.meta.url)),
        },
    },
    build: {
        target: ["chrome109", "edge109", "firefox109", "safari16.3"],
        outDir: "public/build",
        emptyOutDir: true,
        manifest: true,
        rollupOptions: {
            input: {
                site: "resources/js/site.ts",
            },
            output: {
                entryFileNames: "[name]-[hash].js",
                chunkFileNames: "[name]-[hash].js",
                assetFileNames: "[name]-[hash].[ext]",
            },
        },
    },
});
