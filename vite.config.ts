import { defineConfig } from "vite";
import tailwindcss from "@tailwindcss/vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
    publicDir: false,
    plugins: [tailwindcss(), vue()],
    build: {
        outDir: "public",
        emptyOutDir: false,
        rollupOptions: {
            input: {
                app: "resources/js/app.ts",
                styles: "resources/css/app.css",
                prose: "resources/css/prose.css",
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
