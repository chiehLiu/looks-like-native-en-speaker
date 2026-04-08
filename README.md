# LooksLikeANativeEnSpeaker

A desktop utility that rewrites your English sentences to sound more native. Powered by Gemini AI.

## Features

- Paste any English text and get 3 rewrites: **Natural**, **Professional**, and **Casual**
- One-click copy to clipboard
- Always-on-top transparent window for quick access
- Global shortcut: `Cmd+Shift+L` to toggle the window
- Keyboard shortcuts: `Cmd+Enter` to rewrite, `Esc` to hide
- Draggable borderless window

## Tech Stack

- **Tauri v2** — native desktop shell
- **Vue 3** — reactive UI with Composition API
- **Vite** — fast dev server and bundler
- **Tailwind CSS v4** — utility-first styling
- **Gemini API** — AI-powered sentence rewriting (free tier)
- **pnpm** — fast, disk-efficient package manager

## Prerequisites

- [Rust](https://www.rust-lang.org/learn/get-started#installing-rust)
- [Node.js](https://nodejs.org/) (v18+)
- [pnpm](https://pnpm.io/)

## Setup

```bash
# Clone and install
cd looks-like-native-en-speaker
pnpm install

# Add your API key
cp .env.example .env
# Edit .env and set your API key (see below)

# Run in dev mode
source ~/.cargo/env && pnpm tauri dev
```

## Switching AI Provider

The app calls an AI API from the Tauri Rust backend. To switch providers, update two files:

### 1. Set your API key in `.env`

```bash
# Google Gemini (default, free tier available)
GEMINI_API_KEY=your-gemini-key

# To get a free Gemini key:
# Go to https://aistudio.google.com/apikey → Create API Key
```

Note: The API key is read by the Rust backend at runtime via `std::env::var`, so it is never embedded in the frontend JS bundle.

### 2. Change the model in `apps/desktop/src-tauri/src/lib.rs`

Find the API URL line and swap the model name:

```rust
// Current: Gemini 2.5 Flash (free)
"https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}"

// Other options:
// gemini-2.5-flash (free, 5 RPM)
// gemini-2.0-flash (if your account has quota)
```

To switch to a completely different provider (Claude, OpenAI), you'll need to update the request format and response parsing in `lib.rs`, and the env variable name in `lib.rs`.

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
