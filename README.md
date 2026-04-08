# LooksLikeANativeEnSpeaker

A desktop utility that rewrites your English sentences to sound more native. Powered by Claude AI.

## Features

- Paste any English text and get 3 rewrites: **Natural**, **Professional**, and **Casual**
- One-click copy to clipboard
- Always-on-top window for quick access
- Global shortcut: `Cmd+Shift+F` to toggle the window
- Keyboard shortcuts: `Cmd+Enter` to rewrite, `Esc` to hide

## Tech Stack

- **Tauri v2** — native desktop shell
- **Vue 3** — reactive UI with Composition API
- **Vite** — fast dev server and bundler
- **Tailwind CSS v4** — utility-first styling
- **Claude API** — AI-powered sentence rewriting
- **pnpm** — fast, disk-efficient package manager

## Prerequisites

- [Rust](https://www.rust-lang.org/learn/get-started#installing-rust)
- [Node.js](https://nodejs.org/) (v18+)
- [pnpm](https://pnpm.io/)
- A [Claude API key](https://console.anthropic.com/)

## Setup

```bash
# Clone and install
cd looks-like-native-en-speaker
pnpm install

# Add your API key
cp .env.example .env
# Edit .env and set VITE_ANTHROPIC_API_KEY=sk-ant-...

# Run in dev mode
pnpm tauri dev
```

## Project Structure

```
looks-like-native-en-speaker/
  apps/
    desktop/          # Tauri + Vue 3 desktop app
      src/            # Vue frontend
      src-tauri/      # Rust backend
  .env                # API key (not committed)
  pnpm-workspace.yaml
```
