import { defineConfig } from "vite";
import module from "./module.json";

export default defineConfig({
    build: {
        lib: {
            entry: "src/index.ts",
            formats: ["es"],
            fileName: module.id,
        },
        sourcemap: true,
    },
});
