# CLAUDE.md

## Project Overview

Tauri v2 desktop markdown editor for blog writing.
Single binary, cross-platform (Windows / macOS / Linux).

## Stack

| Layer       | Technology                |
| ----------- | ------------------------- |
| Frontend    | Svelte 5 + Vite           |
| Editor      | CodeMirror 6              |
| MD pipeline | unified / remark / rehype |
| Backend     | Rust + Tauri v2           |

## Directory Structure

```
src/                        # Svelte frontend
  lib/
    editor/                 # CodeMirror setup
    preview/                # unified pipeline
    components/
src-tauri/
  src/
```

## Commands

```bash
# dev
pnpm tauri dev

# build (single binary)
pnpm tauri build
```
