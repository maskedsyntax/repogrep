# Repogrep

Local Code Snippet Manager — a cross-platform desktop app for managing, tagging, and searching code snippets. Built with Tauri (Rust) and Vue 3.

## Features

- **Snippet management** — Add, edit, delete snippets with title, code, language, and tags
- **Full-text search** — Fast search with SQLite FTS5; filter by tags
- **Index directories** — Scan folders for code files (e.g. `.rs`, `.js`, `.py`) and index as snippets
- **Import / Export** — Paste from clipboard, export to JSON
- **Syntax highlighting** — Preview with Highlight.js
- **Dark / light theme** — Toggle in the header
- **Keyboard shortcut** — Press `/` to focus search

## Setup

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/)
- Linux: WebKitGTK and other [Tauri dependencies](https://v2.tauri.app/start/linux/)

### Install and run

```bash
npm install
npm run tauri dev
```

### Build for production

```bash
npm run tauri build
```

Installers will be in `src-tauri/target/release/bundle/`.

## Project structure

- `src/` — Vue 3 frontend (Vite)
- `src-tauri/` — Rust backend and Tauri config
- Snippets and DB are stored in your OS app data directory (e.g. `~/.local/share/com.repogrep.app/` on Linux).

## License

MIT
