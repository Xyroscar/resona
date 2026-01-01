# Resona

Resona is a desktop API client for testing and debugging HTTP APIs. Built with Tauri, SvelteKit, and Rust.

## Features

- **Workspaces**: Organize your API requests into workspaces with tags for easy filtering
- **Collections**: Group related requests into collections within workspaces
- **Variables**: Define variables at global, workspace, collection, or request scope with automatic interpolation
- **HTTP Client**: Send HTTP requests with support for various body types (JSON, form-data, URL-encoded, etc.)
- **Sync Groups**: Sync variables across multiple workspaces
- **Themes**: Multiple built-in themes including light, dark, and Catppuccin variants (Latte, Frappe, Macchiato, Mocha)
- **Persistent Storage**: All data is stored locally using an embedded database (redb)

## Tech Stack

- **Frontend**: SvelteKit, TypeScript, TailwindCSS, shadcn-svelte
- **Backend**: Rust, Tauri
- **Database**: redb (embedded key-value store)
- **HTTP**: reqwest

## Development

### Prerequisites

- Node.js (v18+)
- Rust (latest stable)
- Bun (or npm/yarn/pnpm)

### Setup

```bash
# Install dependencies
bun install

# Run in development mode
bun run tauri dev

# Build for production
bun run tauri build
```

### Signing the application on macOS

```bash
export APPLE_SIGNING_IDENTITY="<YOUR_SIGNING_IDENTITY>"
bun run tauri build
```

#### To get the signing identity

```bash
security find-identity -v -p codesigning
```

### Reason for not including appimage in the targets

Due to an issue with linuxdeploy on my system, I wasn't able to build the appimage so I removed it from the targets. 

### Project Structure

```
resona/
├── src/                    # Frontend (SvelteKit)
│   ├── lib/
│   │   ├── components/     # UI components
│   │   ├── services/       # API service layer
│   │   └── types/          # TypeScript types
│   └── routes/             # SvelteKit routes
├── src-tauri/              # Backend (Rust/Tauri)
│   └── src/
│       ├── collections/    # Collections module
│       ├── db/             # Database layer
│       ├── http/           # HTTP client
│       ├── requests/       # Requests module
│       ├── settings/       # App settings
│       ├── variables/      # Variables module
│       └── workspaces/     # Workspaces module
└── static/                 # Static assets
```

## License

MIT License - see [LICENSE](LICENSE) for details.
