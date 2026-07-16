import { fileURLToPath } from "node:url";

import { defineConfig } from "vitest/config";

// Vitest config kept separate from vite.config.ts so the Vitest type surface
// does not clash with the Tauri build plugins. Tests are pure logic, so the
// default Node environment is enough — no jsdom needed.
export default defineConfig({
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  test: {
    environment: "node",
    include: ["src/**/*.test.ts"],
  },
});
