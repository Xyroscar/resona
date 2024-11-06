# Resona - API Client

Resona is a modern, cross-platform API client built with Tauri, SvelteKit, and TypeScript. It provides a powerful interface for testing and managing API requests with features like workspace management, collections, and environment variables.

## Features

- 🚀 Cross-platform desktop application (Windows, macOS, Linux)
- 📁 Workspace-based organization
- 📑 Request collections
- 🔄 Environment variables support
- 🎨 Beautiful UI with DaisyUI components
- 🌓 Light/Dark theme support
- 💾 Local storage persistence
- ⚡ Fast and lightweight

## Tech Stack

- **Frontend**: SvelteKit + TypeScript
- **Backend**: Tauri (Rust)
- **Styling**: TailwindCSS + DaisyUI
- **Database**: SQLite (via rusqlite)
- **Build Tool**: Vite

## Prerequisites

- Node.js (v16 or later)
- Bun
- Rust (latest stable)
- Cargo
- System dependencies for Tauri (see [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites))

## Installation

1. Clone the repository and install dependencies: 
```bash
git clone https://github.com/Xyroscar/resona.git
cd resona
bun install
```

2. For development server: 
```bash
bun run tauri dev
```

This will launch both the SvelteKit development server and the Tauri application.

## Building

To create a production build:
```bash
bun run tauri build
```

The built application will be available in the `src-tauri/target/release` directory.


## Configuration

### Development Server

The development server runs on port 1420 by default. You can modify this in the `vite.config.ts` file.

### Tauri

Tauri configuration can be found in `src-tauri/tauri.conf.json`. This includes window settings, build configurations, and security policies.

### Styling

The application uses TailwindCSS with DaisyUI for styling. Configuration can be found in:
- `tailwind.config.js` - TailwindCSS configuration
- `src/app.css` - Global styles

## Acknowledgments

- [Tauri](https://tauri.app/) - For the desktop application framework
- [SvelteKit](https://kit.svelte.dev/) - For the frontend framework
- [DaisyUI](https://daisyui.com/) - For the UI components