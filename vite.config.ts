import { defineConfig } from "vite";
import tailwindcss from "@tailwindcss/vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
    publicDir: "resources/static",
    plugins: [tailwindcss(), vue()],
    build: {
        outDir: "public",
        emptyOutDir: false,
        watch: {
            exclude: ["public/**"],
        },
        rollupOptions: {
            input: {
                app: "resources/js/app.ts",
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
