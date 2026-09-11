import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
    publicDir: "resources/static",
    plugins: [vue()],
    build: {
        outDir: "public",
        emptyOutDir: false,
        rollupOptions: {
            input: {
                alpine: "resources/js/alpine.ts",
                ascii: "resources/js/ascii/index.ts",
                islands: "resources/js/islands.ts",
            },
            output: {
                entryFileNames: "assets/[name].js",
                chunkFileNames: "assets/[name]-[hash].js",
                assetFileNames: "assets/[name].[ext]",
            },
        },
    },
});
