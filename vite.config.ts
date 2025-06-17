import { defineConfig } from "vite";
import module from "./module.json" with { type: "json" };

export default defineConfig({
    build: {
        lib: {
            entry: "ts/index.ts",
            formats: ["es"],
            fileName: module.id,
        },
        sourcemap: true,
    },
});
