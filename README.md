<p align="center">
  <img src="public/app-icon.svg" width="140" alt="AIRust_MT Logo" />
</p>

<h1 align="center">AIRust_MT</h1>

<p align="center">
  A WYSIWYG Markdown desktop editor rebuilt with <b>Rust + Tauri 2</b><br />
  Recreating the MarkText writing experience with a smaller footprint and lower resource usage
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-v0.1.3-1f6feb" alt="版本 v0.1.3" />
  <img src="https://img.shields.io/badge/license-MIT-31a354" alt="MIT License" />
  <img src="https://img.shields.io/badge/Rust-Tauri%202-ff6b6b" alt="Tauri 2" />
  <img src="https://img.shields.io/badge/Vue-3%20%2B%20TypeScript-42b883" alt="Vue 3 + TypeScript" />
  <img src="https://img.shields.io/badge/AI-vibe_coding-ff6b6b" alt="vibe-coding" />
  <img src="https://img.shields.io/badge/editor-alloy1987-7048e8" alt="alloy1987" />
</p>

> The vast majority of this project's code was written by the author through **vibe coding** — using [DeepSeek V4 Flash](https://www.deepseek.com/) and [Qwen 3.8](https://www.qianwenai.com/) as coding models, and [opencode](https://opencode.ai) as the AI programming agent, under human design and review. See [Development Approach](#development-approach) below.

---

## 📖 Table of Contents

- [✨ Features](#features)
- [📦 Installation](#installation)
- [🔐 Privacy](#privacy)
- [⚙️ Tech Stack](#tech-stack)
- [⌨️ Keyboard Shortcuts](#keyboard-shortcuts)
- [🧱 Project Structure](#project-structure)
- [🛠️ Building](#building)
- [🤝 Acknowledgements](#acknowledgements)
- [🤖 Development Approach](#development-approach)
- [🧑‍💻 Author's Note](#authors-note)
- [📜 License](#license)

## ✨ Features

- **WYSIWYG editing**: built on the MarkText editor core `@muyajs/core`, rendering instantly as you type — no split-pane preview needed
- **Rich block elements**: headings, lists, tables, code blocks (syntax highlighting), math formulas (KaTeX), diagrams (Mermaid / flowchart / PlantUML / Vega), HTML blocks, Front Matter, and more
- **Multi-tab**: edit multiple documents at once, with unsaved-change reminders
- **File sidebar**: open a folder as a workspace with a file tree for browsing, creating, renaming, and deleting
- **Plain text editing**: beyond Markdown, open and edit common plain-text files as raw text (no parsing): data & config (.json / .yaml / .yml / .xml / .toml / .ini / .csv / .env), documents & web pages (.txt / .html / .htm / .css / .rtf), and source code (.py / .js / .ts / .java / .c / .cpp / .go / .rs); the current file extension is shown in the status bar
- **Smart text detection**: whether a file can be opened is decided by its content, not its extension — text files with unknown extensions open just fine, while binary files are detected and rejected with a warning
- **Outline panel**: quick navigation by heading levels
- **Find / Replace**: supports regular expressions, case sensitivity, and whole-word matching
- **Image support**: paste or drag images to automatically save them to the document directory and insert them
- **File watching**: automatic prompts when a document is modified externally on disk
- **Encoding detection**: automatic detection and conversion of non-UTF-8 file encodings based on `encoding_rs` + `chardetng`
- **Large file handling**: very large files open in read-only preview mode to avoid freezing
- **12 theme skins**: Bright White, Dark Black, Indigo, Emerald Green, Sunset Orange, Deep Sea Blue, Rose Pink, Dawn Gold, Mint, Sky Blue, Peach Pink, Lavender
- **7 interface languages**: 中文, English, 日本語, Русский, 한국어, Español, Français
- **UI zoom**: Ctrl + scroll / menu zoom, optimized for high-DPI screens
- **Single instance**: focusing the existing window and opening files when launched again
- **Windows NSIS installer**: multilingual setup wizard, supports opening files via drag-and-drop

## 📦 Installation

### Windows (installer .exe)

- Before installing, make sure the **Microsoft Edge WebView2 Runtime** is installed on your system;
- During installation, the installer automatically detects whether WebView2 is installed:
  - If installed, installation continues directly;
  - If not, the installer will prompt you and automatically download and install WebView2;
- You can also download and install WebView2 manually in advance. Official download page: <https://developer.microsoft.com/microsoft-edge/webview2/>

### macOS and Linux

> Installers for macOS and Linux are not published yet, but you can build the app from source. See [Building](#building) below.

## 🔐 Privacy

> The application runs **entirely locally and offline**, with the following diagram features as exceptions, which require an internet connection when rendering:
>
> - **PlantUML diagrams**: diagram source code is sent to the public server `plantuml.com` for rendering and the resulting image is returned; diagram content leaves your machine;
> - **Sequence diagrams (sequence)**: fonts are loaded from Google Fonts via webfontloader during rendering.
>
> All other features (editing, saving, images, encoding detection, etc.) make no network requests. If your document content is confidential, avoid using the two diagram types above.

## ⚙️ Tech Stack

| Layer         | Technology                                                                                                                                                                                   |
| ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Desktop shell | [Tauri 2](https://tauri.app) + Rust (`tauri-plugin-dialog` / `tauri-plugin-opener` / `tauri-plugin-single-instance`, `notify` file watching, `encoding_rs` + `chardetng` encoding detection) |
| Frontend      | Vue 3 + TypeScript + Vite + Pinia                                                                                                                                                            |
| Editor core   | [@muyajs/core](https://github.com/marktext/marktext) (muya from MarkText, snabbdom virtual DOM rendering)                                                                                    |
| File icons    | @marktext/file-icons (from MarkText)                                                                                                                                                         |

## ⌨️ Keyboard Shortcuts

| Shortcut                        | Function       |
| ------------------------------- | -------------- |
| `Ctrl + N`                      | New document   |
| `Ctrl + O`                      | Open file      |
| `Ctrl + Shift + O`              | Open folder    |
| `Ctrl + S`                      | Save           |
| `Ctrl + Shift + S`              | Save as        |
| `Ctrl + F`                      | Find / Replace |
| `Ctrl + Alt + F`                | Toggle sidebar |
| `Ctrl + A`                      | Select all     |
| `Ctrl + Z`                      | Undo           |
| `Ctrl + Shift + Z` / `Ctrl + Y` | Redo           |
| `Ctrl + 0`                      | Zoom to 100%   |
| `Ctrl + scroll`                 | Zoom UI        |

> On macOS, use `Cmd` instead of `Ctrl`.

## 🧱 Project Structure

```
AIRust_MT/
├── src/                  # Frontend (Vue 3 + TypeScript + Pinia)
│   ├── components/       #   UI components (sidebar, tab bar, search, dialogs, etc.)
│   ├── stores/           #   State management (editor, theme, zoom)
│   ├── editor/           #   muya core adaptation layer
│   └── api.ts            #   Tauri command wrappers
├── src-tauri/            # Desktop shell (Rust + Tauri 2)
│   ├── src/              #   Commands, file watching, encoding detection, large file handling, menus
│   └── nsis/             #   NSIS installer hook scripts
├── editor/               # Core and icon packages from MarkText
│   ├── muya/             #   @muyajs/core WYSIWYG core
│   └── file-icons/       #   @marktext/file-icons file icons
└── public/               # Static assets
```

## 🛠️ Building

Prerequisites: [Rust](https://www.rust-lang.org/), [Node.js](https://nodejs.org/) ≥ 20, [pnpm](https://pnpmjs.com/), and [Tauri 2 system dependencies](https://tauri.app/start/prerequisites/).

Platform-specific prerequisites:

- **Windows**: [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) (usually preinstalled);
- **Linux**: install the system packages listed in the [official Tauri prerequisites](https://tauri.app/start/prerequisites/), e.g. `webkit2gtk-4.1`, `libappindicator3`, `librsvg2-dev`;
- **macOS**: [Xcode Command Line Tools](https://developer.apple.com/xcode/) (`xcode-select --install`).

```bash
# Install dependencies
pnpm install

# Development mode (hot reload)
pnpm tauri dev

# Build release installer
pnpm tauri build
```

Output installers by platform:

- Windows: `src-tauri/target/release/bundle/nsis/*.exe`
- macOS: `src-tauri/target/release/bundle/macos/*.app` and `dmg/*.dmg`
- Linux: `src-tauri/target/release/bundle/deb/*.deb`, `rpm/*.rpm`, `appimage/*.AppImage`

## 🤝 Acknowledgements

This project referenced the following open-source projects during development. Thanks to their authors and contributors. The full text of the relevant licenses is included in the "Third-Party Notices" section of the [LICENSE](LICENSE) file.

### [MarkText](https://github.com/marktext/marktext) (MIT License)

This project is a rewrite of MarkText, and the following parts come directly from MarkText:

| Referenced part                   | Location in this project                      | Description                                                                        |
| --------------------------------- | --------------------------------------------- | ---------------------------------------------------------------------------------- |
| muya editor core                  | `editor/muya/` (`@muyajs/core`)               | Core editing capabilities: WYSIWYG editing, block rendering, shortcuts, formatting |
| File icons                        | `editor/file-icons/` (`@marktext/file-icons`) | Sidebar file tree icons                                                            |
| Product form & interaction design | Global                                        | WYSIWYG writing philosophy, feature scope, and interaction patterns                |

### [Markpad](https://github.com/sftwrdotdev/Markpad) (BSD 3-Clause License)

Many technical details of the desktop side (Tauri 2) referenced Markpad, mainly including:

| Referenced part                 | Description                                                                                             |
| ------------------------------- | ------------------------------------------------------------------------------------------------------- |
| Tauri 2 app architecture        | Rust-side command organization and frontend-backend communication                                       |
| Native menus & event dispatch   | Menu definitions and forwarding events to the frontend (see `src-tauri/src/menu.rs`)                    |
| Packaging & installer practices | NSIS installer configuration, install hook scripts (`src-tauri/nsis/`), and other engineering practices |

### Special Thanks

Here I would like to give **special thanks** to Mozilla and the Rust Foundation — their efforts have given us an excellent programming language: Rust!
In the wave of AI-assisted programming, Rust has become a highly promising development language largely due to its four core advantages as a low-level systems language:

1. **Ultimate execution efficiency and low-level control**
   As a systems programming language close to the hardware, Rust abandons the traditional garbage collection (GC) mechanism and provides zero-cost abstractions. This allows it to achieve C/C++-level runtime performance and memory control while maintaining development efficiency, perfectly fitting the AI era's demands for high-performance computing and high-concurrency processing.

2. **Rigorous memory safety and reliability guarantees**
   Rust is known for its rigorous syntax and its unique ownership and borrow checker mechanisms. It can precisely intercept memory safety hazards such as null pointers and data races at compile time. This "safe at compile time" feature provides a strong quality backstop for AI-generated code, greatly reducing the risk of runtime crashes.

3. **A strong type system as a "semantic constraint" for AI**
   Rust has a highly standardized and rigorous strong type system. In the context of AI programming, this type system is not only a code specification but also a "navigator" for AI. Clear type definitions help AI more accurately understand business logic and data flow, effectively reducing invalid code produced by AI "hallucinations" or logic flaws, making AI-generated code inherently more robust.

4. **The official compiler as a "strict quality inspector"**
   Rust officially provides a mature and extremely strict compiler toolchain. In AI-assisted development workflows, AI is responsible for quickly generating code drafts, while the Rust compiler acts as the first strict quality gate. As long as AI-generated code compiles, it means that most fatal errors in memory safety and type matching have been eliminated. This complementary model of "AI produces, compiler inspects" greatly improves the delivery quality of industrial-grade code.
   **In summary, in the era of AI-assisted programming, choosing Rust as a development language has extremely high strategic value. I also hope to see more capable people join the Rust programming community and make the Rust ecosystem ever richer.**

## 🤖 Development Approach

This project is an **AI-assisted programming (vibe coding)** practice:

- **Coding model**: [DeepSeek V4 Flash](https://www.deepseek.com/) / [Qwen 3.8](https://www.qianwenai.com/)
- **Image model**: [Qwen 3.8](https://www.qianwenai.com/)
- **Programming agents**: [opencode](https://opencode.ai) (interactive AI programming CLI), [Qwencode](https://www.qianwenai.com)
- **Human role**: requirements definition, architecture decisions, code review, and acceptance testing

The project uses MarkText as its blueprint, replacing its Electron shell with Rust + Tauri 2, while the editor core continues to use and adapt MarkText's muya.

## 🧑‍💻 Author's Note

**A cross-disciplinary engineer**: I've long worked in the financial industry. Not a CS graduate, but passionate about programming. Self-taught Python and built practical tools, Codewars Python 5kyu.

**A vibe coding practitioner**: AI lets ordinary people cross the programming threshold. I believe that in the AI era, the ceiling of AI is the ceiling of your imagination — everyone can use AI to chase their dreams.

**An open-source newbie**: this is my first GitHub project, so there will inevitably be rough edges and shortcomings.

**Looking forward to connecting**: suggestions in Issues or PRs are very welcome — help me grow together!

**E-mail：** 20360505@qq.com

## 📜 License

This project is released as a whole under the **[MIT License](LICENSE)**.

Because this project is derived from / references MarkText (MIT) and Markpad (BSD 3-Clause), to meet the compliance requirements of these two licenses:

- `editor/muya/` and `editor/file-icons/` retain MarkText's MIT copyright notice;
- The [LICENSE](LICENSE) file includes a "Third-Party Notices" section containing the complete original copyright notices and license texts of MarkText and Markpad;
- The in-app "Help → About" and "Help → License" menus also display the attribution information above.

---

<p align="center"><i>AIRust_MT — reimagining a classic with AI and Rust, making Markdown writing light again.</i></p>
