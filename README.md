# Another WoW Addon Manager

[![PR Build (Windows)](https://github.com/Zannnol/another-wow-addon-manager/actions/workflows/pr-windows.yml/badge.svg?branch=main)](https://github.com/Zannnol/another-wow-addon-manager/actions/workflows/pr-windows.yml)
[![Release Build (Multi-OS)](https://github.com/Zannnol/another-wow-addon-manager/actions/workflows/release.yml/badge.svg)](https://github.com/Zannnol/another-wow-addon-manager/actions/workflows/release.yml)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-blue.svg)](https://github.com/Zannnol/another-wow-addon-manager)
[![Built with Tauri v2](https://img.shields.io/badge/built%20with-Tauri%20v2-24C8DB.svg)](https://tauri.app/)
[![Privacy First](https://img.shields.io/badge/privacy-100%25%20local-success.svg)](#philosophie)

Addon and mod manager for World of Warcraft (Retail, Classic, Forever, and custom flavors) designed for **Windows**, **macOS**, and **Linux** with a strict **privacy-first** approach.

---

## What is does (or doesn't)

- **No Chromium / No Electron**: built with **Tauri v2** (Rust backend + native OS WebView) to stay lightweight, fast, and resource-efficient.
- **100% local & privacy-first**:
  - No user accounts, no analytics, no telemetry SDKs, and no background tracking.
  - No network calls at startup; external requests only occur on explicit user action (search, update checks, downloads).
  - Local state, profiles, backups, and addon index stored locally on disk (SQLite + local config/snapshots).
- **Multi-flavor support**: manage multiple World of Warcraft installations (`Interface/AddOns` and `WTF`) across different versions and flavors.

---

## Technical

- **Desktop & Core**: Rust + Tauri v2
- **Local database**: SQLite (`rusqlite` with `bundled` feature)
- **Frontend**: TypeScript + lightweight UI framework (Svelte / Vite) + TailwindCSS
- **CI/CD**: GitHub Actions (Windows PR builds and manual multi-OS release builds)

---

## Development prerequisites

### Common tools
- **Rust** (`rustup`, `rustc`, `cargo`)
- **Node.js** (LTS) et **pnpm**

### System Dependencies by Operating System
- **Windows**: Microsoft C++ Build Tools (MSVC) + WebView2 (included on Windows 10/11)
- **macOS**: Xcode Command Line Tools
- **Linux (Debian/Ubuntu)**: `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `libgtk-3-dev`, `build-essential`, `curl`, `wget`, `file`, `libssl-dev`, `pkg-config`

---

## Quick Start

### 1. Clone the repository
```bash
git clone [https://github.com/Zannnol/another-wow-addon-manager.git](https://github.com/Zannnol/another-wow-addon-manager.git)
cd another-wow-addon-manager
```

### 2. Install the frontend dependencies
```bash
pnpm install
```

### 3. Launch the app in development mode
```bash
pnpm tauri dev
```

### 4. Run the Rust unit tests
```bash
cd src-tauri
cargo test --lib
```

---

## GitHub Actions Workflows

- **PR Build (Windows)** (`.github/workflows/pr-windows.yml`): runs on pull requests targeting `main` and builds Windows installers/artifacts for testing.
- **Release Build (Multi-OS)** (`.github/workflows/release.yml`): triggered manually (`workflow_dispatch`) to build across `windows-latest`, `ubuntu-latest`, and `macos-latest` (`aarch64-apple-darwin` and `x86_64-apple-darwin`).

---

## Project Structure

```text
another-wow-addon-manager/
├── .github/workflows/   # CI / CD workflows
├── src/                 # Frontend (TypeScript + UI)
├── src-tauri/           # Rust backend, SQLite, Tauri config & capabilities
├── package.json
└── README.md
```

---

## Licence

GPL-3.0