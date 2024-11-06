import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";

export default defineConfig(async () => ({
    plugins: [sveltekit()],

    clearScreen: false,

    server:{
        port: 1420,
        strictPort: true
    },

    envPrefix: ["VITE_", "TAURI_"],
}))