<p align="center">
  <img src="public/app-icon.svg" width="140" alt="AIRust_MT Logo" />
</p>

<h1 align="center">AIRust_MT</h1>

<p align="center">
  使用 <b>Rust + Tauri 2</b> 重构的所见即所得（WYSIWYG）Markdown 桌面编辑器<br />
  复刻 MarkText 的写作体验，拥有更小的体积与更低的资源占用
</p>

<div>

</div>

<p align="center">
  <img src="https://img.shields.io/badge/version-v0.1.3-1f6feb" alt="版本 v0.1.3" />
  <img src="https://img.shields.io/badge/license-MIT-31a354" alt="MIT License" />
  <img src="https://img.shields.io/badge/Rust-Tauri%202-ff6b6b" alt="Tauri 2" />
  <img src="https://img.shields.io/badge/Vue-3%20%2B%20TypeScript-42b883" alt="Vue 3 + TypeScript" />
  <img src="https://img.shields.io/badge/AI-vibe_coding-ff6b6b" alt="vibe-coding" />
  <img src="https://img.shields.io/badge/editor-alloy1987-7048e8" alt="alloy1987" />
</p>

> 本项目的绝大部分代码由作者通过 **vibe coding** 方式完成——即以 [DeepSeek V4 Flash](https://www.deepseek.com/) 与 [Qwen 3.8](https://www.qianwenai.com/) 作为编码模型、[opencode](https://opencode.ai) 作为 AI 编程代理（agent），在人工设计与审查下生成。详见下文[开发方式](#开发方式)。

---

## 📖 目录

- [✨ 功能特性](#功能特性)
- [📦 安装](#安装)
- [🔐 隐私说明](#隐私说明)
- [⚙️ 技术栈](#技术栈)
- [⌨️ 快捷键](#快捷键)
- [🧱 项目结构](#项目结构)
- [🛠️ 构建](#构建)
- [🤝 参考项目与致谢](#参考项目与致谢)
- [🤖 开发方式](#开发方式)
- [🧑‍💻 作者自白](#作者自白)
- [📜 许可证](#许可证)

## ✨ 功能特性

- **所见即所得编辑**：基于 MarkText 的编辑器内核 `@muyajs/core`，写作时即时渲染，无需分栏预览
- **丰富的块级元素**：标题、列表、表格、代码块（语法高亮）、数学公式（KaTeX）、图表（Mermaid / flowchart / PlantUML / Vega）、HTML 块、Front Matter 等
- **多标签页**：同时编辑多个文档，未保存更改提醒
- **文件侧边栏**：打开文件夹作为工作区，文件树浏览、新建、重命名、删除
- **纯文本编辑**：除 Markdown 外，还可以按纯文本（不解析）打开和编辑常见的纯文本文件：数据与配置文件（.json / .yaml / .yml / .xml / .toml / .ini / .csv / .env）、文档与网页（.txt / .html / .htm / .css / .rtf）以及源代码（.py / .js / .ts / .java / .c / .cpp / .go / .rs）；状态栏会显示当前文件的扩展名
- **智能文本识别**：文件能否打开由内容而非扩展名决定——未知扩展名的文本文件可以正常打开，而二进制文件会被识别并以警告提示拒绝打开
- **大纲面板**：按标题层级快速导航
- **查找 / 替换**：支持正则表达式、区分大小写、全字匹配
- **图片支持**：粘贴、拖拽图片自动保存到文档目录并插入
- **文件监听**：文档在磁盘上被外部修改时自动提示
- **编码识别**：基于 `encoding_rs` + `chardetng` 自动检测并转换非 UTF-8 文件编码
- **大文件处理**：超大文件先以只读预览方式打开，避免卡顿
- **12 套主题皮肤**：明亮白、暗夜黑、靛蓝、翡翠绿、日落橙、深海蓝、玫瑰粉、晨曦金、薄荷青、晴空蓝、蜜桃粉、薰衣草
- **7 种界面语言**：中文、English、日本語、Русский、한국어、Español、Français
- **界面缩放**：Ctrl + 滚轮 / 菜单缩放，适配高分屏
- **单实例运行**：重复启动时聚焦已有窗口并打开文件
- **Windows NSIS 安装包**：多语言安装向导，支持文件拖拽打开

## 📦 安装

### Windows（安装包 .exe）

- 安装前请确保系统已安装 **Microsoft Edge WebView2 运行时**（即 web2view，简称 WebView2）；
- 安装过程中，安装程序会自动检测 WebView2 是否已安装：
  - 若已安装，将直接继续安装；
  - 若未安装，安装程序会给出提示，并自动联网下载、安装 WebView2；
- 您也可以提前手动下载安装 WebView2，官方下载地址：<https://developer.microsoft.com/microsoft-edge/webview2/>

### macOS 与 Linux

> macOS 与 Linux 的安装包尚未发布，但可以通过源码自行构建，参见下文[构建](#构建)章节。

## 🔐 隐私说明

> 本应用整体为**本地离线运行**，但以下图表功能例外，渲染时需要联网：
>
> - **PlantUML 图表**：图表源码会发送到公共服务器 `plantuml.com` 渲染后返回图片，图表内容会离开本机；
> - **时序图（sequence）**：渲染时通过 webfontloader 从 Google Fonts 加载字体。
>
> 其余功能（编辑、保存、图片、编码检测等）均不产生任何网络请求。若文档内容涉密，请避免使用上述两类图表。

## ⚙️ 技术栈

| 层         | 技术                                                                                                                                                                           |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 桌面外壳   | [Tauri 2](https://tauri.app) + Rust（`tauri-plugin-dialog` / `tauri-plugin-opener` / `tauri-plugin-single-instance`、`notify` 文件监听、`encoding_rs` + `chardetng` 编码检测） |
| 前端       | Vue 3 + TypeScript + Vite + Pinia                                                                                                                                              |
| 编辑器内核 | [@muyajs/core](https://github.com/marktext/marktext)（源自 MarkText 的 muya，snabbdom 虚拟 DOM 渲染）                                                                          |
| 文件图标   | @marktext/file-icons（源自 MarkText）                                                                                                                                          |

## ⌨️ 快捷键

| 快捷键                          | 功能        |
| ------------------------------- | ----------- |
| `Ctrl + N`                      | 新建文档    |
| `Ctrl + O`                      | 打开文件    |
| `Ctrl + Shift + O`              | 打开文件夹  |
| `Ctrl + S`                      | 保存        |
| `Ctrl + Shift + S`              | 另存为      |
| `Ctrl + F`                      | 查找 / 替换 |
| `Ctrl + Alt + F`                | 切换侧边栏  |
| `Ctrl + A`                      | 全选        |
| `Ctrl + Z`                      | 撤销        |
| `Ctrl + Shift + Z` / `Ctrl + Y` | 重做        |
| `Ctrl + 0`                      | 缩放 100%   |
| `Ctrl + 滚轮`                   | 界面缩放    |

> macOS 下以 `Cmd` 替代 `Ctrl`。

## 🧱 项目结构

```
AIRust_MT/
├── src/                  # 前端（Vue 3 + TypeScript + Pinia）
│   ├── components/       #   界面组件（侧边栏、标签栏、搜索、对话框等）
│   ├── stores/           #   状态管理（编辑器、主题、缩放）
│   ├── editor/           #   muya 内核适配层
│   └── api.ts            #   Tauri 命令封装
├── src-tauri/            # 桌面外壳（Rust + Tauri 2）
│   ├── src/              #   命令、文件监听、编码检测、大文件处理、菜单
│   └── nsis/             #   NSIS 安装器钩子脚本
├── editor/               # 源自 MarkText 的内核与图标包
│   ├── muya/             #   @muyajs/core 所见即所得内核
│   └── file-icons/       #   @marktext/file-icons 文件图标
└── public/               # 静态资源
```

## 🛠️ 构建

前置要求：[Rust](https://www.rust-lang.org/)、[Node.js](https://nodejs.org/) ≥ 20、[pnpm](https://pnpmjs.com/)，以及 [Tauri 2 的系统依赖](https://tauri.app/start/prerequisites/)。

各平台额外要求：

- **Windows**：[Microsoft Edge WebView2 运行时](https://developer.microsoft.com/microsoft-edge/webview2/)（通常系统已自带）；
- **Linux**：安装 [Tauri 官方前置依赖](https://tauri.app/start/prerequisites/) 中列出的系统包，如 `webkit2gtk-4.1`、`libappindicator3`、`librsvg2-dev`；
- **macOS**：[Xcode Command Line Tools](https://developer.apple.com/xcode/)（`xcode-select --install`）。

```bash
# 安装依赖
pnpm install

# 开发模式（热重载）
pnpm tauri dev

# 构建发布版安装包
pnpm tauri build
```

各平台构建产物：

- Windows：`src-tauri/target/release/bundle/nsis/*.exe`
- macOS：`src-tauri/target/release/bundle/macos/*.app` 与 `dmg/*.dmg`
- Linux：`src-tauri/target/release/bundle/deb/*.deb`、`rpm/*.rpm`、`appimage/*.AppImage`

## 🤝 参考项目与致谢

本项目在开发过程中参考了以下开源项目，在此向它们的作者与贡献者致谢。相关许可证全文收录于 [LICENSE](LICENSE) 文件的「第三方声明」部分。

### [MarkText](https://github.com/marktext/marktext)（MIT License）

本项目是 MarkText 的重构版本，以下部分直接来源于 MarkText：

| 参考部分           | 本项目位置                                     | 说明                                                       |
| ------------------ | ---------------------------------------------- | ---------------------------------------------------------- |
| muya 编辑器内核    | `editor/muya/`（`@muyajs/core`）               | 所见即所得编辑、块级元素渲染、快捷键、格式化等核心编辑能力 |
| 文件图标           | `editor/file-icons/`（`@marktext/file-icons`） | 侧边栏文件树图标                                           |
| 产品形态与交互设计 | 全局                                           | 所见即所得的写作理念、功能范围与交互方式                   |

### [Markpad](https://github.com/sftwrdotdev/Markpad)（BSD 3-Clause License）

桌面端（Tauri 2）的诸多技术细节参考了 Markpad，主要包括：

| 参考部分           | 说明                                                           |
| ------------------ | -------------------------------------------------------------- |
| Tauri 2 应用架构   | Rust 端命令组织、前后端通信方式                                |
| 原生菜单与事件分发 | 菜单项定义及向前端转发事件的写法（见 `src-tauri/src/menu.rs`） |
| 打包与安装器实践   | NSIS 安装器配置、安装钩子脚本（`src-tauri/nsis/`）等工程实践   |

### 特别感谢

在此，我要**特别感谢**Mozilla与Rust Foundation（Rust基金会），他们的努力为我们提供了一门非常优秀的计算机语言Rsut！
在AI辅助编程的浪潮中，Rust之所以成为极具潜力的开发语言，主要得益于其作为底层系统级语言所具备的四大核心优势：

1. **极致的执行效率与底层控制力**
   作为一门贴近底层的系统级编程语言，Rust摒弃了传统语言的垃圾回收（GC）机制，提供了零成本抽象。这使得它在保证开发效率的同时，能够实现媲美C/C++的极高运行性能与内存控制力，完美契合AI时代对高性能计算与高并发处理的需求。

2. **严谨的内存安全与可靠性保障**
   Rust以其严谨的语法和独特的所有权（Ownership）与借用检查（Borrow Checker）机制著称。它能在编译阶段就精准拦截空指针、数据竞争等内存安全隐患。这种“编译期即安全”的特性，为AI生成的代码提供了强大的质量兜底，大幅降低了运行时崩溃的风险。

3. **强类型系统对AI的“语义约束”**
   Rust拥有高度标准化且严谨的强类型系统。在AI编程语境下，这种类型系统不仅是代码的规范，更是AI的“导航仪”。明确的类型定义能帮助AI更准确地理解业务逻辑与数据流向，有效减少AI因“幻觉”或逻辑漏洞产生的无效代码，使AI生成的代码天然具备更高的健壮性。

4. **官方编译器作为“严苛的质检员”**
   Rust官方提供了一套成熟且极其严格的编译器工具链。在AI辅助开发的工作流中，AI负责快速生成代码初稿，而Rust编译器则充当第一道严苛的质检关卡。只要AI生成的代码能够通过编译，就意味着其在内存安全和类型匹配上已经排除了绝大多数致命错误。这种“AI负责产出，编译器负责质检”的互补模式，极大地提升了工业级代码的交付质量。
   **综上所述，在AI辅助编程的时代，选择Rust作为开发语言具有极高的战略价值。我也希望看到更多有能力的人加入到Rust编程大军中，让Rust生态越来越丰富。**

## 🤖 开发方式

本项目是一次 **AI 辅助编程（vibe coding）** 实践：

- **编码模型**：[DeepSeek V4 Flash](https://www.deepseek.com/) / [Qwen 3.8](https://www.qianwenai.com/)
- **图片模型**：[Qwen 3.8](https://www.qianwenai.com/)
- **编程代理**：[opencode](https://opencode.ai)（交互式 AI 编程 CLI）、[Qwencode](https://www.qianwenai.com)
- **人的角色**：需求定义、架构决策、代码审查与验收测试

项目以 MarkText 为蓝本，用 Rust + Tauri 2 替换其 Electron 外壳，编辑器内核则直接沿用并适配 MarkText 的 muya。

## 🧑‍💻 作者自白

**跨界理工男**：长期从事金融行业，非科班出身，但热爱编程。曾自学 Python 并开发过实用小工具，Codewars Python 5kyu。

**Vibe Coding 践行者**：AI 让普通人跨越了编程的门槛。我相信在 AI 时代，AI 的上限就是你想象力的上限，每个人都能用 AI 追逐梦想。

**开源新手**：这是我的首个 GitHub 项目，难免有青涩与不足。

**期待交流**：非常欢迎大家在 Issues 中提出改进建议或 PR，帮助我一起成长！

**E-mail：** 20360505@qq.com

## 📜 许可证

本项目整体采用 **[MIT License](LICENSE)** 发布。

由于本项目派生/参考了 MarkText（MIT）与 Markpad（BSD 3-Clause），为满足这两个许可证的合规要求：

- `editor/muya/` 与 `editor/file-icons/` 保留 MarkText 的 MIT 版权声明；
- [LICENSE](LICENSE) 文件中附有「第三方声明（Third-Party Notices）」章节，完整收录 MarkText 与 Markpad 的原始版权声明与许可证全文；
- 应用内「帮助 → 关于」与「帮助 → 许可证」菜单中同样展示了上述归属信息。

---

<p align="center"><i>AIRust_MT —— 用AI与Rust重构经典，让 Markdown 写作轻装上阵。</i></p>
