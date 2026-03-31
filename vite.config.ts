import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  build: {
    lib: {
      entry: "src/index.ts",
      formats: ["es"],
      fileName: "wayfinder",
    },
    sourcemap: true,
  },
  plugins: [wasm()],
});
