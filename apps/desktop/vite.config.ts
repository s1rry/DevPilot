import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

// Vite config tuned for Tauri development.
// https://tauri.app/start/frontend/vite/
export default defineConfig({
  plugins: [react(), tailwindcss()],

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
