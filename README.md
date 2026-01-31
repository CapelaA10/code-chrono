# Code Chrono 🕐💻

Privacy-first desktop task & time tracker for developers and tech-savvy users. Pomodoro timers, GitHub/Jira sync, local-first storage. Cross-platform: **Mac • Linux • Windows**.

![Demo GIF or screenshot](https://via.placeholder.com/800x400?text=Code+Chrono+Demo) <!-- Add later -->

## ✨ Features
- **Pomodoro Timer** with idle detection & notifications
- **Task Management** (CRUD, projects, tags, drag-drop)
- **Time Tracking** with CSV reports & GitHub issue sync
- **Global Hotkeys** for quick actions
- **Offline-First** SQLite + WebDAV sync
- **Extensible** plugin system

## 🛠 Tech Stack
- **Frontend**: Svelte [web:45]
- **Backend**: Rust + Tauri (5MB binaries) [web:2]
- **Storage**: SQLite [web:72]
- **License**: MIT [web:91]

## 🚀 Quick Start

### Prerequisites
- Node.js 18+
- Rust (via `rustup`)

### Installation
```bash
git clone https://github.com/CapelaA10/code-chrono
cd code-chrono
npm install

# Install Rust if needed: 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


npm run tauri dev
