# Code Chrono 🕐💻

Privacy-first desktop task & time tracker for developers and tech-savvy users. Pomodoro timers, GitHub/Jira sync, local-first storage. Cross-platform: **Mac • Linux • Windows**.

## ✨ Features
- **Pomodoro Timer** with idle detection & notifications
- **Task Management** with simple name
- **Time Tracking** with CSV reports
- **Global Hotkeys** for quick actions
- **Offline-First** SQLite

## 🛠 Tech Stack
- **Frontend**: Svelte [web:45]
- **Backend**: Rust + Tauri (5MB binaries) [web:2]
- **Storage**: SQLite [web:72]
- **License**: MIT [web:91]

## 🚀 Quick Start

### Prerequisites
- Node.js
- Rust (via `rustup`)

### Installation
```bash
git clone https://github.com/CapelaA10/code-chrono
cd code-chrono
npm install

# Install Rust if needed: 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Run dev mode
npm run tauri dev
```
## 🤝 Contributing

We love contributions! Whether you're a Rustacean, a Svelte wizard, or just found a typo, here's how you can help:

1. **Fork** the repository.
2. **Create a branch** for your feature or fix (`git checkout -b feature/cool-new-thing`).
3. **Commit your changes** (`git commit -m 'Add some cool feature'`).
4. **Push to the branch** (`git push origin feature/cool-new-thing`).
5. **Open a Pull Request**.

### 🛠 Development Focus
- **Rust/Tauri**: Optimizing SQLite queries and system-level notifications.
- **Svelte**: Improving the UI/UX of the timer and task list.
- **Integrations**: We are looking to add more sync options (e.g., GitLab, Notion).

Please make sure to update tests as appropriate.