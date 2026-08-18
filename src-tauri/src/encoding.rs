//! 编码检测、编码、round-trip 无损验证。
//!
//! 三层检测策略：BOM → UTF-8 验证 → chardetng 智能猜测。
//! 编码时拒绝不可映射字符（防止 emoji 被写坏），保存前做无损验证。

/// 解码结果
#[derive(Debug)]
pub struct DecodedText {
    pub content: String,
    /// 是否发生了信息丢失（存在不可解码字节）
    pub lossy: bool,
    /// 检测到的编码名称（用于回写）
    pub encoding: String,
}

/// 编码标签常量（与前端约定）
pub const UTF8_LABEL: &str = "utf-8";
pub const UTF8_BOM_LABEL: &str = "utf-8-bom";
pub const UTF16LE_LABEL: &str = "utf-16le";
pub const UTF16BE_LABEL: &str = "utf-16be";

/// 编码错误特殊标记（前端据此展示 i18n 消息）
pub const UNMAPPABLE_CODE: &str = "ENCODING_UNMAPPABLE";
/// 原文件解码有信息丢失，拒绝覆盖保存
pub const LOSSY_CODE: &str = "LOSSY_SAVE_BLOCKED";

/// 三层编码检测：BOM → UTF-8 验证 → chardetng 猜测
pub fn decode_text(bytes: &[u8]) -> DecodedText {
    // 1. BOM 检测（最准确）
    if let Some((encoding, bom_len)) = encoding_rs::Encoding::for_bom(bytes) {
        let label: String = if encoding == encoding_rs::UTF_8 {
            UTF8_BOM_LABEL.to_owned()
        } else {
            encoding.name().to_lowercase()
        };
        let (content, had_errors) = encoding.decode_without_bom_handling(&bytes[bom_len..]);
        return DecodedText {
            content: content.into_owned(),
            lossy: had_errors,
            // 解码出错时强制按 UTF-8 兜底，避免把损坏内容原样回写
            encoding: if had_errors { UTF8_LABEL.to_owned() } else { label },
        };
    }

    // 2. UTF-8 验证（纯 ASCII / 合法 UTF-8 直接通过）
    if let Ok(content) = std::str::from_utf8(bytes) {
        return DecodedText {
            content: content.to_owned(),
            lossy: false,
            encoding: UTF8_LABEL.to_owned(),
        };
    }

    // 3. chardetng 智能猜测（覆盖 Shift_JIS / EUC-KR / Big5 / GBK 等）
    let mut detector = chardetng::EncodingDetector::new();
    detector.feed(bytes, true);
    let encoding = detector.guess(None, false);
    let (content, _, had_errors) = encoding.decode(bytes);
    DecodedText {
        content: content.into_owned(),
        lossy: had_errors,
        // name() 返回大写形式（如 "UTF-16LE"），统一转小写作为回写标签
        encoding: encoding.name().to_lowercase(),
    }
}

/// 把字符串编码为指定编码的字节。
///
/// 关键：拒绝不可映射的字符（防止 emoji 被写成 `&#128512;` 或替换符）。
pub fn encode_text(content: &str, label: &str) -> Result<Vec<u8>, String> {
    let utf16 = |big_endian: bool| -> Result<Vec<u8>, String> {
        let mut bytes = if big_endian {
            vec![0xFE, 0xFF]
        } else {
            vec![0xFF, 0xFE]
        };
        for unit in content.encode_utf16() {
            let pair = if big_endian {
                unit.to_be_bytes()
            } else {
                unit.to_le_bytes()
            };
            bytes.extend_from_slice(&pair);
        }
        Ok(bytes)
    };

    match label {
        UTF8_LABEL => Ok(content.as_bytes().to_vec()),
        UTF8_BOM_LABEL => {
            let mut bytes = vec![0xEF, 0xBB, 0xBF];
            bytes.extend_from_slice(content.as_bytes());
            Ok(bytes)
        }
        UTF16LE_LABEL => utf16(false),
        UTF16BE_LABEL => utf16(true),
        _ => {
            // 任意编码都用 encoding_rs；拒绝「会自动转码」的编码
            let encoding = encoding_rs::Encoding::for_label(label.as_bytes())
                .filter(|e| e.output_encoding() == *e)
                .ok_or_else(|| format!("Unknown text encoding: {}", label))?;

            let (bytes, _, unmappable) = encoding.encode(content);
            if unmappable {
                return Err(UNMAPPABLE_CODE.to_owned());
            }
            Ok(bytes.into_owned())
        }
    }
}

/// Round-trip 无损验证：encode → decode 必须等于原内容。
pub fn verify_lossless_roundtrip(content: &str, encoding: &str) -> Result<(), String> {
    let bytes = encode_text(content, encoding)?;
    let decoded = decode_text(&bytes);
    if decoded.content != content {
        let label = if encoding.is_empty() {
            "自动检测".to_string()
        } else {
            encoding.to_uppercase()
        };
        Err(format!(
            "编码安全：文档内容包含无法用 {} 无损编码的字符，直接覆盖将损坏原文件。\n\
             已拒绝保存。请使用「另存为」保存为 UTF-8 等能完整表示的编码。",
            label
        ))
    } else {
        Ok(())
    }
}

/// 智能切分 UTF-8 边界（防止截断半个字符）。
pub fn utf8_truncation_boundary(bytes: &[u8]) -> usize {
    let len = bytes.len();
    for back in 1..=3.min(len) {
        let index = len - back;
        let byte = bytes[index];
        if byte & 0b1100_0000 == 0b1000_0000 {
            continue;
        }
        let needed = if byte & 0b1000_0000 == 0 {
            1
        } else if byte & 0b1110_0000 == 0b1100_0000 {
            2
        } else if byte & 0b1111_0000 == 0b1110_0000 {
            3
        } else if byte & 0b1111_1000 == 0b1111_0000 {
            4
        } else {
            return len;
        };
        return if back < needed { index } else { len };
    }
    len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_utf8() {
        let bytes = "你好世界".as_bytes();
        let result = decode_text(bytes);
        assert_eq!(result.content, "你好世界");
        assert!(!result.lossy);
        assert_eq!(result.encoding, UTF8_LABEL);
    }

    #[test]
    fn test_decode_utf8_bom() {
        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend_from_slice("你好".as_bytes());
        let result = decode_text(&bytes);
        assert_eq!(result.content, "你好");
        assert!(!result.lossy);
        assert_eq!(result.encoding, UTF8_BOM_LABEL);
    }

    #[test]
    fn test_decode_utf16le() {
        let mut bytes = vec![0xFF, 0xFE];
        for unit in "你好".encode_utf16() {
            bytes.extend_from_slice(&unit.to_le_bytes());
        }
        let result = decode_text(&bytes);
        assert_eq!(result.content, "你好");
        assert_eq!(result.encoding, UTF16LE_LABEL);
    }

    #[test]
    fn test_decode_utf16be() {
        let mut bytes = vec![0xFE, 0xFF];
        for unit in "你好".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        let result = decode_text(&bytes);
        assert_eq!(result.content, "你好");
        assert_eq!(result.encoding, UTF16BE_LABEL);
    }

    #[test]
    fn test_decode_gbk() {
        // 用 encoding_rs 生成一段较长的 GBK 文本，保证 chardetng 有足够统计量
        let text = "这是一个使用 GBK 编码的中文测试文件，包含多个汉字与中文标点，用来验证自动编码检测的正确性。";
        let bytes = encoding_rs::GBK.encode(text).0;
        let result = decode_text(&bytes);
        assert_eq!(result.content, text);
        assert!(!result.lossy);
        assert!(
            result.encoding.eq_ignore_ascii_case("gbk") || result.encoding.eq_ignore_ascii_case("gb18030"),
            "unexpected encoding: {}",
            result.encoding
        );
    }

    #[test]
    fn test_decode_shift_jis() {
        // 用 encoding_rs 生成一段较长的 Shift_JIS 日文文本
        let text = "こんにちは。これは日本語のテスト文章です。シフトジスエンコードで書かれています。";
        let bytes = encoding_rs::SHIFT_JIS.encode(text).0;
        let result = decode_text(&bytes);
        assert_eq!(result.content, text);
        assert!(!result.lossy);
        assert!(
            result.encoding.to_lowercase().contains("shift_jis"),
            "unexpected encoding: {}",
            result.encoding
        );
    }

    #[test]
    fn test_decode_euc_kr() {
        // 用 encoding_rs 生成一段较长的 EUC-KR 韩文文本
        let text = "안녕하세요. 이것은 한국어 테스트 파일입니다. EUC-KR 인코딩으로 작성되었습니다.";
        let bytes = encoding_rs::EUC_KR.encode(text).0;
        let result = decode_text(&bytes);
        assert_eq!(result.content, text);
        assert!(!result.lossy);
        assert!(
            result.encoding.to_lowercase().contains("euc-kr"),
            "unexpected encoding: {}",
            result.encoding
        );
    }

    #[test]
    fn test_decode_big5() {
        // 用 encoding_rs 生成一段较长的 Big5 繁体中文文本
        let text = "這是一個使用 Big5 編碼的繁體中文測試檔案，用來驗證自動編碼偵測的正確性。";
        let bytes = encoding_rs::BIG5.encode(text).0;
        let result = decode_text(&bytes);
        assert_eq!(result.content, text);
        assert!(!result.lossy);
        assert!(
            result.encoding.to_lowercase().contains("big5"),
            "unexpected encoding: {}",
            result.encoding
        );
    }

    #[test]
    fn test_legacy_roundtrip_via_label() {
        // 解码得到的标签（小写）应能反向编码并 round-trip
        for (text, encoding) in [
            (
                "这是简体中文企业的普通文本。",
                encoding_rs::GBK,
            ),
            (
                "こんにちは、シフトジスです。",
                encoding_rs::SHIFT_JIS,
            ),
        ] {
            let bytes = encoding.encode(text).0;
            let decoded = decode_text(&bytes);
            assert_eq!(decoded.content, text);
            let reencoded = encode_text(&decoded.content, &decoded.encoding).unwrap();
            assert_eq!(decode_text(&reencoded).content, text);
        }
    }

    #[test]
    fn test_encode_unmappable_emoji_to_gbk() {
        // emoji 不能用 GBK 编码，应该报 ENCODING_UNMAPPABLE
        let result = encode_text("你好 😀", "GBK");
        assert_eq!(result.unwrap_err(), UNMAPPABLE_CODE);
    }

    #[test]
    fn test_encode_utf16_roundtrip() {
        let bytes = encode_text("你好 😀", UTF16LE_LABEL).unwrap();
        let decoded = decode_text(&bytes);
        assert_eq!(decoded.content, "你好 😀");
        assert!(!decoded.lossy);
        assert_eq!(decoded.encoding, UTF16LE_LABEL);
    }

    #[test]
    fn test_verify_lossless_rejects_gbk_emoji() {
        // 中文可以无损用 GBK 编码，但混入 emoji 后必须被拒绝
        assert!(verify_lossless_roundtrip("纯中文测试", "GBK").is_ok());
        assert!(verify_lossless_roundtrip("中文 😀", "GBK").is_err());
    }

    #[test]
    fn test_utf8_truncation_boundary() {
        // "你好" = E4 BD A0 E5 A5 BD (6 字节)
        let bytes = "你好".as_bytes();
        // 在第 4 字节截断（"好" 中间），应该回到第 3 字节
        let truncated = utf8_truncation_boundary(&bytes[..4]);
        assert_eq!(truncated, 3);
        // 完整字节不应截断
        assert_eq!(utf8_truncation_boundary(bytes), 6);
    }
}
