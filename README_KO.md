<p align="center">
  <img src="public/app-icon.svg" width="140" alt="AIRust_MT 로고" />
</p>

<h1 align="center">AIRust_MT</h1>

<p align="center">
  <b>Rust + Tauri 2</b>로 재구성한 WYSIWYG(위지위그) Markdown 데스크톱 편집기<br />
  MarkText의 집필 경험을 재현하면서 더 작은 용량과 더 낮은 리소스 사용량을 실현
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-v0.1.3-1f6feb" alt="버전 v0.1.3" />
  <img src="https://img.shields.io/badge/license-MIT-31a354" alt="MIT License" />
  <img src="https://img.shields.io/badge/Rust-Tauri%202-ff6b6b" alt="Tauri 2" />
  <img src="https://img.shields.io/badge/Vue-3%20%2B%20TypeScript-42b883" alt="Vue 3 + TypeScript" />
  <img src="https://img.shields.io/badge/AI-vibe_coding-ff6b6b" alt="vibe-coding" />
  <img src="https://img.shields.io/badge/editor-alloy1987-7048e8" alt="alloy1987" />
</p>

> 본 프로젝트 코드의 대부분은 작성자가 **vibe coding** 방식으로 완성했습니다. 즉 [DeepSeek V4 Flash](https://www.deepseek.com/)와 [Qwen 3.8](https://www.qianwenai.com/)을 코딩 모델로, [opencode](https://opencode.ai)를 AI 프로그래밍 에이전트(agent)로 사용하여 인간의 설계와 검토 아래 생성되었습니다. 자세한 내용은 아래 [개발 방식](#개발-방식)을 참조하세요.

---

## 📖 목차

- [✨ 기능](#기능)
- [📦 설치](#설치)
- [🔐 개인정보 보호](#개인정보-보호)
- [⚙️ 기술 스택](#기술-스택)
- [⌨️ 단축키](#단축키)
- [🧱 프로젝트 구조](#프로젝트-구조)
- [🛠️ 빌드](#빌드)
- [🤝 참고 프로젝트 및 감사의 말](#참고-프로젝트-및-감사의-말)
- [🤖 개발 방식](#개발-방식)
- [🧑‍💻 작성자의 말](#작성자의-말)
- [📜 라이선스](#라이선스)

## ✨ 기능

- **WYSIWYG 편집**: MarkText의 편집기 코어 `@muyajs/core` 기반, 작성 즉시 렌더링되어 분할 미리보기가 필요 없음
- **풍부한 블록 요소**: 제목, 목록, 표, 코드 블록(구문 강조), 수학 공식(KaTeX), 다이어그램(Mermaid / flowchart / PlantUML / Vega), HTML 블록, Front Matter 등
- **다중 탭**: 여러 문서를 동시에 편집, 저장되지 않은 변경 사항 알림
- **파일 사이드바**: 폴더를 작업 공간으로 열어 파일 트리 탐색, 새로 만들기, 이름 바꾸기, 삭제 지원
- **일반 텍스트 편집**: Markdown 외에도 일반 텍스트 파일을 원문 그대로(파싱 없이) 열어 편집할 수 있습니다: 데이터·설정 파일(.json / .yaml / .yml / .xml / .toml / .ini / .csv / .env), 문서·웹 페이지(.txt / .html / .htm / .css / .rtf), 소스 코드(.py / .js / .ts / .java / .c / .cpp / .go / .rs). 현재 파일 확장자는 상태 표시줄에 표시됩니다
- **스마트 텍스트 감지**: 파일을 열 수 있는지 여부는 확장자가 아니라 내용으로 판단됩니다——알 수 없는 확장자의 텍스트 파일은 정상적으로 열리지만, 바이너리 파일은 감지되어 경고와 함께 열기가 거부됩니다
- **개요 패널**: 제목 단계별 빠른 탐색
- **찾기 / 바꾸기**: 정규식, 대소문자 구분, 단어 단위 일치 지원
- **이미지 지원**: 붙여넣기, 드래그한 이미지를 문서 디렉터리에 자동 저장 후 삽입
- **파일 감시**: 문서가 디스크에서 외부 수정될 때 자동 알림
- **인코딩 감지**: `encoding_rs` + `chardetng` 기반으로 비 UTF-8 파일 인코딩 자동 감지 및 변환
- **대용량 파일 처리**: 초대형 파일은 읽기 전용 미리보기 모드로 열어 끊김 방지
- **12가지 테마 스킨**: 밝은 화이트, 어두운 블랙, 인디고, 에메랄드 그린, 선셋 오렌지, 딥 씨 블루, 로즈 핑크, 던 골드, 민트, 스카이 블루, 피치 핑크, 라벤더
- **7가지 인터페이스 언어**: 中文, English, 日本語, Русский, 한국어, Español, Français
- **인터페이스 확대/축소**: Ctrl + 휠 / 메뉴 확대, 고해상도 화면 대응
- **단일 인스턴스 실행**: 중복 실행 시 기존 창에 포커스를 맞추고 파일 열기
- **Windows NSIS 설치 패키지**: 다국어 설치 마법사, 파일 드래그 앤 드롭 열기 지원

## 📦 설치

### Windows(설치 패키지 .exe)

- 설치 전 시스템에 **Microsoft Edge WebView2 런타임**(web2view, 약칭 WebView2)이 설치되어 있는지 확인하세요.
- 설치 과정에서 설치 프로그램이 WebView2 설치 여부를 자동으로 감지합니다:
  - 이미 설치된 경우 바로 설치를 계속합니다.
  - 설치되지 않은 경우 설치 프로그램이 안내를 표시하고 자동으로 인터넷에서 WebView2를 다운로드하여 설치합니다.
- 사전에 수동으로 WebView2를 다운로드하여 설치할 수도 있습니다. 공식 다운로드 주소: <https://developer.microsoft.com/microsoft-edge/webview2/>

### macOS 및 Linux

> macOS와 Linux용 설치 패키지는 아직 공개되지 않았지만, 소스 코드에서 직접 빌드할 수 있습니다. 자세한 내용은 아래의 [빌드](#빌드) 섹션을 참조하세요.

## 🔐 개인정보 보호

> 본 애플리케이션은 전반적으로 **로컬 오프라인 실행**되지만, 다음 다이어그램 기능은 예외로 렌더링 시 인터넷 연결이 필요합니다:
>
> - **PlantUML 다이어그램**: 다이어그램 소스 코드가 공용 서버 `plantuml.com`으로 전송되어 렌더링된 이미지가 반환되며, 다이어그램 내용이 로컬 기기를 벗어납니다.
> - **시퀀스 다이어그램(sequence)**: 렌더링 시 webfontloader를 통해 Google Fonts에서 글꼴을 로드합니다.
>
> 그 외 기능(편집, 저장, 이미지, 인코딩 감지 등)은 어떠한 네트워크 요청도 발생시키지 않습니다. 문서 내용이 기밀인 경우 위의 두 가지 다이어그램 사용을 피하세요.

## ⚙️ 기술 스택

| 계층        | 기술                                                                                                                                                                             |
| ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 데스크톱 셸 | [Tauri 2](https://tauri.app) + Rust(`tauri-plugin-dialog` / `tauri-plugin-opener` / `tauri-plugin-single-instance`, `notify` 파일 감시, `encoding_rs` + `chardetng` 인코딩 감지) |
| 프런트엔드  | Vue 3 + TypeScript + Vite + Pinia                                                                                                                                                |
| 편집기 코어 | [@muyajs/core](https://github.com/marktext/marktext)(MarkText에서 유래한 muya, snabbdom 가상 DOM 렌더링)                                                                         |
| 파일 아이콘 | @marktext/file-icons(MarkText에서 유래)                                                                                                                                          |

## ⌨️ 단축키

| 단축키                          | 기능                 |
| ------------------------------- | -------------------- |
| `Ctrl + N`                      | 새 문서              |
| `Ctrl + O`                      | 파일 열기            |
| `Ctrl + Shift + O`              | 폴더 열기            |
| `Ctrl + S`                      | 저장                 |
| `Ctrl + Shift + S`              | 다른 이름으로 저장   |
| `Ctrl + F`                      | 찾기 / 바꾸기        |
| `Ctrl + Alt + F`                | 사이드바 전환        |
| `Ctrl + A`                      | 전체 선택            |
| `Ctrl + Z`                      | 실행 취소            |
| `Ctrl + Shift + Z` / `Ctrl + Y` | 다시 실행            |
| `Ctrl + 0`                      | 100% 확대/축소       |
| `Ctrl + 휠`                     | 인터페이스 확대/축소 |

> macOS에서는 `Ctrl` 대신 `Cmd`를 사용합니다.

## 🧱 프로젝트 구조

```
AIRust_MT/
├── src/                  # 프런트엔드(Vue 3 + TypeScript + Pinia)
│   ├── components/       #    UI 컴포넌트(사이드바, 탭 바, 검색, 대화상자 등)
│   ├── stores/           #    상태 관리(편집기, 테마, 확대/축소)
│   ├── editor/           #    muya 코어 적응 계층
│   └── api.ts            #    Tauri 명령 래퍼
├── src-tauri/            # 데스크톱 셸(Rust + Tauri 2)
│   ├── src/              #    명령, 파일 감시, 인코딩 감지, 대용량 파일 처리, 메뉴
│   └── nsis/             #    NSIS 설치기 훅 스크립트
├── editor/               # MarkText에서 유래한 코어와 아이콘 패키지
│   ├── muya/             #    @muyajs/core WYSIWYG 코어
│   └── file-icons/       #    @marktext/file-icons 파일 아이콘
└── public/               # 정적 리소스
```

## 🛠️ 빌드

필수 요건: [Rust](https://www.rust-lang.org/), [Node.js](https://nodejs.org/) ≥ 20, [pnpm](https://pnpmjs.com/), 그리고 [Tauri 2 시스템 종속성](https://tauri.app/start/prerequisites/).

플랫폼별 추가 요건:

- **Windows**: [Microsoft Edge WebView2 런타임](https://developer.microsoft.com/microsoft-edge/webview2/)(일반적으로 시스템에 사전 설치됨);
- **Linux**: [Tauri 공식 사전 요구사항](https://tauri.app/start/prerequisites/)에 나열된 시스템 패키지(예: `webkit2gtk-4.1`, `libappindicator3`, `librsvg2-dev`) 설치;
- **macOS**: [Xcode Command Line Tools](https://developer.apple.com/xcode/)(`xcode-select --install`).

```bash
# 의존성 설치
pnpm install

# 개발 모드(핫 리로드)
pnpm tauri dev

# 릴리스 설치 패키지 빌드
pnpm tauri build
```

플랫폼별 빌드 산출물:

- Windows: `src-tauri/target/release/bundle/nsis/*.exe`
- macOS: `src-tauri/target/release/bundle/macos/*.app` 및 `dmg/*.dmg`
- Linux: `src-tauri/target/release/bundle/deb/*.deb`, `rpm/*.rpm`, `appimage/*.AppImage`

## 🤝 참고 프로젝트 및 감사의 말

본 프로젝트는 개발 과정에서 아래 오픈소스 프로젝트를 참고했습니다. 이 자리를 빌려 작성자와 기여자 여러분께 감사의 말씀을 전합니다. 관련 라이선스 전문은 [LICENSE](LICENSE) 파일의 「서드파티 고지(Third-Party Notices)」 섹션에 수록되어 있습니다.

### [MarkText](https://github.com/marktext/marktext)(MIT License)

본 프로젝트는 MarkText의 리팩터링 버전이며, 다음 부분은 MarkText에서 직접 유래했습니다:

| 참고 부분                   | 본 프로젝트 위치                             | 설명                                                           |
| --------------------------- | -------------------------------------------- | -------------------------------------------------------------- |
| muya 편집기 코어            | `editor/muya/`(`@muyajs/core`)               | WYSIWYG 편집, 블록 요소 렌더링, 단축키, 서식 등 핵심 편집 기능 |
| 파일 아이콘                 | `editor/file-icons/`(`@marktext/file-icons`) | 사이드바 파일 트리 아이콘                                      |
| 제품 형태와 인터랙션 디자인 | 전역                                         | WYSIWYG 집필 이념, 기능 범위, 인터랙션 방식                    |

### [Markpad](https://github.com/sftwrdotdev/Markpad)(BSD 3-Clause License)

데스크톱(Tauri 2)의 많은 기술적 세부 사항은 Markpad를 참고했으며, 주요 내용은 다음과 같습니다:

| 참고 부분                       | 설명                                                                                  |
| ------------------------------- | ------------------------------------------------------------------------------------- |
| Tauri 2 앱 아키텍처             | Rust 측 명령 구성, 프런트엔드-백엔드 통신 방식                                        |
| 네이티브 메뉴와 이벤트 디스패치 | 메뉴 항목 정의 및 프런트엔드로 이벤트를 전달하는 작성법(`src-tauri/src/menu.rs` 참조) |
| 패키징과 설치기 실무            | NSIS 설치기 구성, 설치 훅 스크립트(`src-tauri/nsis/`) 등 엔지니어링 실무              |

### 특별 감사

여기서 **Mozilla와 Rust Foundation(Rust 재단)에 특별한 감사**를 전합니다. 그들의 노력 덕분에 우리는 매우 훌륭한 프로그래밍 언어인 Rust를 갖게 되었습니다!
AI 지원 프로그래밍의 물결 속에서 Rust가 잠재력 높은 개발 언어가 된 것은 저수준 시스템 언어로서 지닌 네 가지 핵심 장점 덕분입니다:

1. **극한의 실행 효율과 저수준 제어력**
   하드웨어에 가까운 시스템 프로그래밍 언어인 Rust는 전통 언어의 가비지 컬렉션(GC) 메커니즘을 버리고 제로 코스트 추상화를 제공합니다. 이로 인해 개발 효율을 유지하면서 C/C++에 필적하는 매우 높은 실행 성능과 메모리 제어력을 실현하여, AI 시대의 고성능 컴퓨팅과 높은 동시성 처리 요구에 완벽히 부합합니다.

2. **엄격한 메모리 안전성과 신뢰성 보장**
   Rust는 엄격한 문법과 독특한 소유권(Ownership) 및 빌림 검사(Borrow Checker) 메커니즘으로 유명합니다. 컴파일 단계에서 널 포인터, 데이터 레이스 등 메모리 안전 위협을 정확히 차단합니다. 이 "컴파일 단계에서의 안전성" 특성은 AI가 생성한 코드에 강력한 품질 안전망을 제공하여 런타임 충돌 위험을 크게 낮춥니다.

3. **강타입 시스템의 AI에 대한 "의미적 제약"**
   Rust는 고도로 표준화되고 엄격한 강타입 시스템을 갖추고 있습니다. AI 프로그래밍 맥락에서 이 타입 시스템은 코드의 규범일 뿐만 아니라 AI의 "내비게이터"입니다. 명확한 타입 정의는 AI가 비즈니스 로직과 데이터 흐름을 더 정확히 이해하도록 도와, AI의 "환각"이나 논리 허점으로 인한 무효 코드를 효과적으로 줄여 AI가 생성한 코드가 본질적으로 더 높은 견고성을 갖게 합니다.

4. **공식 컴파일러라는 "엄격한 품질 검사관"**
   Rust는 공식적으로 성숙하고 매우 엄격한 컴파일러 툴체인을 제공합니다. AI 지원 개발 워크플로에서 AI는 코드 초안을 빠르게 생성하고, Rust 컴파일러는 첫 번째 엄격한 품질 검사 관문 역할을 합니다. AI가 생성한 코드가 컴파일을 통과했다는 것은 메모리 안전성과 타입 일치에서 대부분의 치명적 오류가 제거되었음을 의미합니다. "AI는 생산을 담당하고, 컴파일러는 품질 검사를 담당한다"는 이 상호보완 모델은 산업급 코드의 납품 품질을 크게 높입니다.
   **요컨대, AI 지원 프로그래밍 시대에 Rust를 개발 언어로 선택하는 것은 매우 높은 전략적 가치를 지닙니다. 더 많은 유능한 분들이 Rust 프로그래밍 대열에 합류하여 Rust 생태계가 더욱 풍성해지기를 바랍니다.**

## 🤖 개발 방식

본 프로젝트는 **AI 지원 프로그래밍(vibe coding)** 실천입니다:

- **코딩 모델**: [DeepSeek V4 Flash](https://www.deepseek.com/) / [Qwen 3.8](https://www.qianwenai.com/)
- **이미지 모델**: [Qwen 3.8](https://www.qianwenai.com/)
- **프로그래밍 에이전트**: [opencode](https://opencode.ai)(인터랙티브 AI 프로그래밍 CLI), [Qwencode](https://www.qianwenai.com)
- **인간의 역할**: 요구사항 정의, 아키텍처 결정, 코드 리뷰, 인수 테스트

프로젝트는 MarkText를 청사진으로 삼아 Electron 셸을 Rust + Tauri 2로 교체했으며, 편집기 코어는 MarkText의 muya를 그대로 계승·적용했습니다.

## 🧑‍💻 작성자의 말

**융합형 이공계 남성**: 오랫동안 금융 업계에서 일했으며, 정통 컴퓨터과학 전공자는 아니지만 프로그래밍을 사랑합니다. Python을 독학하여 실용적인 작은 도구를 개발했고, Codewars Python 5kyu입니다.

**vibe coding 실천자**: AI는 보통 사람들이 프로그래밍의 문턱을 넘게 해주었습니다. 저는 AI 시대에 AI의 상한선이 곧 당신의 상상력의 상한선이며, 누구나 AI로 꿈을 쫓을 수 있다고 믿습니다.

**오픈소스 초보자**: 제 첫 GitHub 프로젝트인 만큼 미숙함과 부족함이 있을 수밖에 없습니다.

**교류 기대**: Issues에 개선 제안이나 PR을 남겨 주시면 대환영입니다. 함께 성장할 수 있도록 도와주세요!

**E-mail：**  20360505@qq.com

## 📜 라이선스

본 프로젝트는 전체적으로 **[MIT License](LICENSE)** 로 배포됩니다.

본 프로젝트는 MarkText(MIT)와 Markpad(BSD 3-Clause)에서 파생/참고했으므로, 이 두 라이선스의 준수 요구를 충족하기 위해:

- `editor/muya/` 와 `editor/file-icons/` 는 MarkText의 MIT 저작권 표기를 유지합니다.
- [LICENSE](LICENSE) 파일에는 「서드파티 고지(Third-Party Notices)」 섹션이 첨부되어 MarkText와 Markpad의 원본 저작권 표기와 라이선스 전문을 완전히 수록하고 있습니다.
- 앱 내 「도움말 → 정보」 및 「도움말 → 라이선스」 메뉴에도 위 귀속 정보가 표시됩니다.

---

<p align="center"><i>AIRust_MT — AI와 Rust로 클래식을 재구성하여 Markdown 집필을 가볍게.</i></p>
