import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

const packageJson = JSON.parse(
  readFileSync(fileURLToPath(new URL("./package.json", import.meta.url)), "utf-8"),
) as { version: string };

// Vite config tuned for Tauri development.
// https://tauri.app/start/frontend/vite/
export default defineConfig({
  plugins: [react(), tailwindcss()],

  // Expose the app version from package.json to the frontend at build time.
  define: {
    __APP_VERSION__: JSON.stringify(packageJson.version),
  },

  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },

  // Tauri expects a fixed port; fail instead of silently picking another one.
  server: {
    port: 1420,
    strictPort: true,
  },

  // Keep the Rust compiler output readable in the same terminal.
  clearScreen: false,

  // Allow Tauri env vars (TAURI_ENV_*) inside the frontend when needed.
  envPrefix: ["VITE_", "TAURI_ENV_"],
});
