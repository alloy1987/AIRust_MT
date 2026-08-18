use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

/// 把路径规范化为统一标识符：
/// - 相对路径 -> 绝对路径
/// - 已存在的文件/目录：`fs::canonicalize`（解析 `..`、符号链接、Windows 磁盘真实大小写、
///   macOS 卷上磁盘实际存储的 Unicode 形式）
/// - 不存在的路径（如另存为尚未落盘）：对最近的已存在祖先 `canonicalize`，尾部追加并做 NFC
///   归一化，保证「café」与「cafe\u0301」这类输入得到同一标识符
pub fn canonicalize_path(p: &str) -> PathBuf {
    let abs = absolute(p);
    let resolved = if let Ok(c) = std::fs::canonicalize(&abs) {
        c
    } else {
        // 路径尚不存在：向上找最近已存在祖先，尾部做 NFC 归一化
        let mut base = abs.clone();
        let mut tail: Vec<std::ffi::OsString> = Vec::new();
        while !base.exists() {
            match base.file_name() {
                Some(n) => {
                    tail.push(n.to_os_string());
                    base.pop();
                }
                None => break,
            }
        }
        let mut out = std::fs::canonicalize(&base).unwrap_or(base);
        for name in tail.iter().rev() {
            out.push(name.to_string_lossy().nfc().collect::<String>());
        }
        out
    };
    strip_windows_prefix(resolved)
}

fn absolute(p: &str) -> PathBuf {
    let pb = PathBuf::from(p);
    if pb.is_absolute() {
        pb
    } else {
        std::env::current_dir().unwrap_or_default().join(pb)
    }
}

/// Windows 上 `fs::canonicalize` 返回带 `\\?\` 前缀的扩展路径，摘掉以保持与用户可见路径一致。
#[cfg(windows)]
fn strip_windows_prefix(p: PathBuf) -> PathBuf {
    let mut s = p.to_string_lossy().into_owned();
    if let Some(rest) = s.strip_prefix(r"\\?\UNC\") {
        s = format!("\\\\{}", rest);
    } else if let Some(rest) = s.strip_prefix(r"\\?\") {
        s = rest.to_string();
    }
    PathBuf::from(s)
}

#[cfg(not(windows))]
fn strip_windows_prefix(p: PathBuf) -> PathBuf {
    p
}