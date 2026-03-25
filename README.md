<p align="center">
  <img src="assets/logo.png" alt="repogrep logo" width="128" />
</p>

<h1 align="center">repogrep</h1>

<p align="center"><strong>Search all your codebases at once.</strong></p>

A lightning-fast, local-first code search tool for developers. No indexing, no cloud, just your code and raw performance.

## The Philosophy

Code search should be simple, private, and incredibly fast. repogrep doesn't use heavy database indexing; instead, it leverages modern hardware and Rust's speed to scan your code in real-time.

Whether you're looking for where an API is used across multiple services or refactoring a shared utility, repogrep gives you the results you need without the overhead.

## Core Features

### Fast Native Search
Built with Rust for maximum performance. Search through thousands of files in milliseconds without pre-indexing. It uses parallelized scanning to make sure you're never waiting on your tools.

### Local & Private
Your code never leaves your machine. No cloud processing, no telemetry, no tracking. Just a powerful local utility that respects your intellectual property and privacy.

### Multi-Repo Search
Add multiple project folders and search across all of them simultaneously. Perfect for microservices, monorepos, or just managing a large collection of independent projects.

### Precise Results
Find exact code snippets, use Regular Expressions for complex patterns, and filter out noise with custom ignore rules. The three-pane interface gives you instant context with syntax-highlighted previews.

---
MIT Licensed.
