<p align="center">
  <img src="assets/logo.png" alt="repogrep logo" width="128" />
</p>

<h1 align="center">repogrep</h1>

<p align="center"><strong>Search all your codebases at once.</strong></p>

A lightning-fast, local-first code search tool for developers. No indexing, no cloud, just your code and raw performance.

## The Philosophy

Code search should be simple, private, and incredibly fast. **repogrep** doesn't use heavy database indexing; instead, it leverages modern hardware and Rust's speed to scan your code in real-time.

Whether you're looking for where an API is used across multiple services or refactoring a shared utility, repogrep gives you the results you need without the overhead.

## Core Features

### ⚡ Fast Native Search
Built with Rust and the industry-standard `ignore` crate for maximum performance. Search through thousands of files in milliseconds without pre-indexing. It uses parallelized scanning via `rayon` to ensure you're never waiting on your tools.

### 🛡️ Local & Private
Your code never leaves your machine. No cloud processing, no telemetry, no tracking. Just a powerful local utility that respects your intellectual property and privacy.

### 📂 Multi-Repo Search
Add multiple project folders and search across all of them simultaneously. Perfect for microservices, monorepos, or just managing a large collection of independent projects.

### 🔍 Precise Results & Context
Find exact code snippets, use Regular Expressions for complex patterns, and filter out noise with custom ignore rules. **repogrep** now shows **context lines** around each match, making it easier to scan results without opening every file.

### 🛠️ Advanced Tooling
- **Smart Git Integration:** Automatically respects `.gitignore` and `.ignore` files.
- **Find & Replace:** Multi-file search and replace with preview support.
- **Tree & List Views:** Toggle between a flat list and a hierarchical folder view.
- **Settings Persistence:** Remembers your preferences (Regex, Case Sensitive, View Mode) across restarts.
- **Export Support:** Save your search results as JSON or CSV for further analysis.

## Getting Started

### Development
1. Install [Rust](https://rustup.rs/) and [Node.js](https://nodejs.org/).
2. Clone the repository.
3. Install dependencies: `npm install`.
4. Run in development mode: `npm run tauri dev`.

### Testing
- **Backend (Rust):** `cd src-tauri && cargo test`
- **Frontend (Vue):** `npm run test`

---
MIT Licensed.
