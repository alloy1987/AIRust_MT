//! 大文件预览与异步加载。
//!
//! 打开文件时先只读取前缀生成预览；保存前必须调用 `ensure_full_content`
//! 加载完整内容，保证不截断保存。

use std::fs;
use std::io::Read;
use std::path::Path;

use serde::Serialize;

use crate::encoding::{decode_text, utf8_truncation_boundary, LOSSY_CODE};

/// 默认预览上限：5 MB
pub const DEFAULT_MAX_BYTES: u64 = 5 * 1024 * 1024;

/// 「加载完整内容」的文件大小上限：100 MB（再大会把整个文件读进内存，有 OOM 风险）
pub const MAX_FULL_LOAD_BYTES: u64 = 100 * 1024 * 1024;

/// 超出完整加载上限的错误码（前端据此展示 i18n 消息）
pub const FULL_LOAD_TOO_LARGE_CODE: &str = "FULL_LOAD_TOO_LARGE";

/// 内容嗅探判定为非纯文本（二进制文件）的错误码（前端据此展示 i18n 消息）
pub const NOT_TEXT_CODE: &str = "NOT_TEXT_FILE";

/// 按 Markdown 解析渲染的扩展名（无扩展名文件也按 Markdown 处理，保持既有行为）。
const MARKDOWN_EXTENSIONS: &[&str] = &[
    "md", "markdown", "mdown", "mkd", "mkdn",
];

/// 内容嗅探：判断字节流是否为二进制（非纯文本）。
///
/// 规则与 git 的二进制判定类似：
/// 1. 带 BOM（UTF-8 / UTF-16LE / UTF-16BE）的直接判为文本——UTF-16 文本天然含大量
///    空字节，必须先于空字节检查处理，否则会被误判；
/// 2. 否则只要含空字节（0x00）即判为二进制——常见文本编码（UTF-8、GBK、Shift_JIS 等）
///    不会出现空字节，而可执行文件/图片/压缩包等二进制格式必然出现。
pub fn is_binary_content(bytes: &[u8]) -> bool {
    if encoding_rs::Encoding::for_bom(bytes).is_some() {
        return false;
    }
    bytes.contains(&0)
}

/// 文件的显示格式："markdown"（所见即所得解析渲染）或 "text"（可编辑纯文本，不解析）。
/// 扩展名属于 Markdown 家族或无扩展名时为 markdown，其余一律按纯文本打开（是否真为
/// 文本由 [`is_binary_content`] 内容嗅探决定）。
pub fn file_format(path: &Path) -> &'static str {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    match name.rsplit_once('.') {
        Some((_, ext)) if MARKDOWN_EXTENSIONS.contains(&ext.to_lowercase().as_str()) => "markdown",
        Some(_) => "text",
        None => "markdown",
    }
}

/// 文件预览结果（打开文件时返回）
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownPreview {
    pub path: String,
    pub name: String,
    pub content: String,
    /// 是否已加载完整文件（false 表示截断预览）
    pub is_full: bool,
    pub lossy: bool,
    pub encoding: String,
    /// 显示格式："markdown"（解析渲染）或 "text"（纯文本编辑），由扩展名决定
    pub format: &'static str,
}

/// 打开文件并生成预览。
///
/// - 文件 ≤ max_bytes：完整读取 + 解码
/// - 文件 > max_bytes：只读前 max_bytes 字节 + 智能切分 UTF-8 边界
pub fn build_markdown_preview(path: &Path, max_bytes: u64) -> Result<MarkdownPreview, String> {
    let path_str = path.to_str().ok_or("Invalid path")?.to_owned();
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();

    let mut f = fs::File::open(path).map_err(|e| format!("读取文件失败: {e}"))?;
    let metadata = f.metadata().map_err(|e| e.to_string())?;
    let fmt = file_format(path);

    // ━━━ 小文件：完整加载 ━━━
    if metadata.len() <= max_bytes {
        let bytes = fs::read(path).map_err(|e| e.to_string())?;
        // 🌟 内容嗅探：二进制文件（exe/png 等，不看扩展名）拒绝当作文本打开
        if is_binary_content(&bytes) {
            return Err(NOT_TEXT_CODE.to_owned());
        }
        let decoded = decode_text(&bytes);
        return Ok(MarkdownPreview {
            path: path_str,
            name,
            content: decoded.content,
            is_full: true,
            lossy: decoded.lossy,
            encoding: decoded.encoding,
            format: fmt,
        });
    }

    // ━━━ 大文件：只读前 max_bytes 字节 ━━━
    let mut vec_buf = Vec::new();
    Read::by_ref(&mut f)
        .take(max_bytes)
        .read_to_end(&mut vec_buf)
        .map_err(|e| e.to_string())?;

    // 修复多字节字符被切断的边界
    vec_buf.truncate(utf8_truncation_boundary(&vec_buf));

    // 大文件同样先嗅前 max_bytes 字节：二进制直接拒绝，避免生成乱码预览
    if is_binary_content(&vec_buf) {
        return Err(NOT_TEXT_CODE.to_owned());
    }

    let preview = decode_text(&vec_buf);
    Ok(MarkdownPreview {
        path: path_str,
        name,
        content: preview.content,
        is_full: false,
        lossy: preview.lossy,
        encoding: preview.encoding,
        format: fmt,
    })
}

/// 完整加载的同步核心：先查文件大小，超限直接拒绝，避免整体读入内存。
fn load_full_content(path: &str, max_bytes: u64) -> Result<String, String> {
    let len = fs::metadata(path).map_err(|e| format!("读取文件失败: {e}"))?.len();
    if len > max_bytes {
        return Err(FULL_LOAD_TOO_LARGE_CODE.to_owned());
    }
    let bytes = fs::read(path).map_err(|e| format!("读取文件失败: {e}"))?;
    let decoded = decode_text(&bytes);
    if decoded.lossy {
        return Err(LOSSY_CODE.to_owned());
    }
    Ok(decoded.content)
}

/// 异步读取完整文件（保存前必须先调用）。
///
/// - 若原文件解码存在信息丢失则拒绝返回，防止覆盖损坏原文件。
/// - 超过 [`MAX_FULL_LOAD_BYTES`] 的超大文件拒绝整体读入内存，防止 OOM。
pub async fn ensure_full_content(path: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || load_full_content(&path, MAX_FULL_LOAD_BYTES))
        .await
        .unwrap_or_else(|e| Err(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file() -> std::path::PathBuf {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "airust_large_file_test_{}_{}.md",
            std::process::id(),
            ts
        ))
    }

    #[test]
    fn test_small_file_full_load() {
        let tmp = temp_file();
        let mut f = fs::File::create(&tmp).unwrap();
        f.write_all(b"# Hello\n\nWorld").unwrap();
        drop(f);

        let preview = build_markdown_preview(&tmp, DEFAULT_MAX_BYTES).unwrap();
        assert!(preview.is_full);
        assert_eq!(preview.content, "# Hello\n\nWorld");
        assert_eq!(preview.encoding, "utf-8");
        assert!(!preview.lossy);
        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_large_file_truncated() {
        let tmp = temp_file();
        let mut f = fs::File::create(&tmp).unwrap();
        // 写约 10 MB 数据
        let chunk = "x".repeat(1024);
        for _ in 0..10240 {
            f.write_all(chunk.as_bytes()).unwrap();
        }
        drop(f);

        let preview = build_markdown_preview(&tmp, 1024 * 1024).unwrap();
        assert!(!preview.is_full);
        assert!(preview.content.len() <= 1024 * 1024);
        assert!(preview.content.ends_with('x'));
        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_large_utf8_not_cut_mid_char() {
        let tmp = temp_file();
        let mut f = fs::File::create(&tmp).unwrap();
        // 大段中文，保证截断点落在多字节字符中间时被修正
        let line = "你好，世界".repeat(2000);
        for _ in 0..40 {
            f.write_all(line.as_bytes()).unwrap();
        }
        drop(f);

        let preview = build_markdown_preview(&tmp, 4096).unwrap();
        assert!(!preview.is_full);
        // 截断边界应落在合法 UTF-8 边界上（结尾无 U+FFFD 替换符）
        assert!(!preview.content.ends_with('\u{FFFD}'), "content was cut mid-char");
        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_load_full_content_ok() {
        let tmp = temp_file();
        fs::write(&tmp, "# Full\n\n完整内容").unwrap();
        let content = load_full_content(&tmp.to_string_lossy(), MAX_FULL_LOAD_BYTES).unwrap();
        assert_eq!(content, "# Full\n\n完整内容");
        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_load_full_content_too_large_rejected() {
        let tmp = temp_file();
        fs::write(&tmp, "x".repeat(1024)).unwrap();
        // 用 512 字节的假上限模拟超大文件，避免测试真写 100 MB
        let err = load_full_content(&tmp.to_string_lossy(), 512).unwrap_err();
        assert_eq!(err, FULL_LOAD_TOO_LARGE_CODE);
        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_is_binary_content() {
        // 普通文本（UTF-8 / 空文件）不是二进制
        assert!(!is_binary_content(b"hello world"));
        assert!(!is_binary_content("你好，世界".as_bytes()));
        assert!(!is_binary_content(b""));
        // UTF-16LE/BE 文本含大量空字节，但带 BOM 必须判为文本
        let mut utf16le = vec![0xFF, 0xFE];
        for unit in "abc".encode_utf16() {
            utf16le.extend_from_slice(&unit.to_le_bytes());
        }
        assert!(!is_binary_content(&utf16le));
        let mut utf16be = vec![0xFE, 0xFF];
        for unit in "abc".encode_utf16() {
            utf16be.extend_from_slice(&unit.to_be_bytes());
        }
        assert!(!is_binary_content(&utf16be));
        // UTF-8 BOM 文本
        let mut utf8bom = vec![0xEF, 0xBB, 0xBF];
        utf8bom.extend_from_slice(b"text");
        assert!(!is_binary_content(&utf8bom));
        // 含空字节且无 BOM：二进制
        assert!(is_binary_content(&[0x4D, 0x5A, 0x00, 0x00]));
        assert!(is_binary_content(b"a\0b"));
    }

    #[test]
    fn test_file_format() {
        use std::path::PathBuf;
        let p = |name: &str| PathBuf::from("/tmp").join(name);
        // Markdown 家族与无扩展名保持解析渲染
        for name in ["a.md", "a.Md", "a.markdown", "a.mdown", "a.mkd", "a.mkdn", "noext"] {
            assert_eq!(file_format(&p(name)), "markdown", "{name}");
        }
        // 其余文本格式为纯文本
        for name in ["a.txt", "a.json", "a.yaml", "a.html", "a.py", "a.rs", ".env", "README.TXT"] {
            assert_eq!(file_format(&p(name)), "text", "{name}");
        }
    }

    #[test]
    fn test_preview_format_field() {
        let tmp = std::env::temp_dir().join(format!(
            "airust_format_test_{}.json",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos()
        ));
        fs::write(&tmp, r#"{"a": 1}"#).unwrap();
        let preview = build_markdown_preview(&tmp, DEFAULT_MAX_BYTES).unwrap();
        assert_eq!(preview.format, "text");
        assert_eq!(preview.content, r#"{"a": 1}"#);
        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_binary_rejected_by_content() {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
        // 故意用文本扩展名：验证拒绝依据是内容而非扩展名
        let tmp = std::env::temp_dir().join(format!("airust_binary_test_{}.txt", ts));
        fs::write(&tmp, [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00]).unwrap();
        let err = build_markdown_preview(&tmp, DEFAULT_MAX_BYTES).unwrap_err();
        assert_eq!(err, NOT_TEXT_CODE);
        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_unknown_extension_text_opens() {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
        // 白名单之外的扩展名（.log）：只要内容是文本就能打开
        let tmp = std::env::temp_dir().join(format!("airust_unknown_ext_test_{}.log", ts));
        fs::write(&tmp, "2026-08-17 info: started").unwrap();
        let preview = build_markdown_preview(&tmp, DEFAULT_MAX_BYTES).unwrap();
        assert_eq!(preview.format, "text");
        assert_eq!(preview.content, "2026-08-17 info: started");
        let _ = fs::remove_file(&tmp);
    }
}
