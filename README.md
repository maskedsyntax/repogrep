# repogrep

Search your project directories for a code snippet. Paste the snippet, pick one or more folders, and see every file that contains it.

## Features

- Add one or more project folders (native dialog). Paths are saved in app data.
- Paste a snippet and search: exact match across all added paths. Enter or Search button.
- Three panes: snippet input, list of matching files (with repo name and path), code preview. Drag the dividers to resize.
- Match list: keyboard up/down, click to open. Preview shows full file with your search term highlighted and scrolls to the first match.
- Many languages supported for search and syntax highlighting (e.g. Rust, Dart, Swift, JS/TS, Python, C/C++, Go, and more).

## Install and run

**Prerequisites:** Node.js 18+, Rust, and on Linux the [Tauri dependencies](https://v2.tauri.app/start/linux/) (e.g. WebKitGTK).

```bash
npm install
npm run tauri dev
```

**Production build:**

```bash
npm run tauri build
```

Binaries and installers end up in `src-tauri/target/release/bundle/`.

## Where your data lives

Project paths are stored in your OS app data directory (e.g. on Linux: `~/.local/share/com.repogrep.app/`). No account, no cloud.

## License

MIT
