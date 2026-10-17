import { defineConfig } from "vite"
import { fileURLToPath } from "node:url"

const PROSEMIRROR = /[\\/]node_modules[\\/](prosemirror-[^\\/]+|orderedmap|rope-sequence|w3c-keyname)[\\/]/

export default defineConfig({
    publicDir: false,
    base: "/build/",

    resolve: {
        alias: {
            "@setups": fileURLToPath(new URL("./bindings/setups.ts", import.meta.url)),
        },
    },
    build: {
        target: ["chrome114", "edge114", "firefox125", "safari17"],
        outDir: "public/build",
        emptyOutDir: true,
        manifest: true,
        rollupOptions: {
            input: {
                site: "resources/js/site.ts",
                editor: "resources/css/editor.css",
            },
            output: {
                entryFileNames: "[name]-[hash].js",
                chunkFileNames: "[name]-[hash].js",
                assetFileNames: "[name]-[hash].[ext]",
                manualChunks: (id) => (PROSEMIRROR.test(id) ? "prosemirror" : undefined),
            },
        },
    },
})
