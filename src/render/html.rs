use crate::net::{Error, Result};
use crate::render::model::{Block, RenderedPage};

#[derive(Debug, Clone)]
pub struct ImageJob {
    pub key: String,
    pub url: url::Url,
}

pub fn render_html(base_url: &url::Url, resp: &crate::net::http::HttpResponse) -> Result<(RenderedPage, Vec<ImageJob>)> {
    // Decode body bytes as text
    let text = decode_html_text(resp)?;

    let mut page = RenderedPage::default();
    let mut images = Vec::new();

    page.title = extract_tag_text(&text, "title");

    // Very small HTML "parser": scan tags and emit blocks for a few common tags.
    // This is intentionally minimal for the assignment scope.
    let mut i = 0;
    let bytes = text.as_bytes();
    let mut current_text = String::new();

    while i < bytes.len() {
        if bytes[i] == b'<' {
            if !current_text.trim().is_empty() {
                page.blocks.push(Block::Paragraph(collapse_ws(&current_text)));
            }
            current_text.clear();

            // Find tag end
            let Some(end) = bytes[i..].iter().position(|&b| b == b'>') else {
                break;
            };
            let tag = &text[i + 1..i + end];
            let tag_l = tag.trim().to_ascii_lowercase();

            if tag_l.starts_with("h1") {
                if let Some((inner, next)) = extract_until_close(&text, i + end + 1, "h1") {
                    page.blocks.push(Block::Heading { level: 1, text: collapse_ws(&inner) });
                    i = next;
                    continue;
                }
            } else if tag_l.starts_with("h2") {
                if let Some((inner, next)) = extract_until_close(&text, i + end + 1, "h2") {
                    page.blocks.push(Block::Heading { level: 2, text: collapse_ws(&inner) });
                    i = next;
                    continue;
                }
            } else if tag_l.starts_with("h3") {
                if let Some((inner, next)) = extract_until_close(&text, i + end + 1, "h3") {
                    page.blocks.push(Block::Heading { level: 3, text: collapse_ws(&inner) });
                    i = next;
                    continue;
                }
            } else if tag_l.starts_with("p") {
                if let Some((inner, next)) = extract_until_close(&text, i + end + 1, "p") {
                    page.blocks.push(Block::Paragraph(collapse_ws(&inner)));
                    i = next;
                    continue;
                }
            } else if tag_l.starts_with("pre") {
                if let Some((inner, next)) = extract_until_close(&text, i + end + 1, "pre") {
                    page.blocks.push(Block::Pre(inner));
                    i = next;
                    continue;
                }
            } else if tag_l.starts_with("br") {
                page.blocks.push(Block::Paragraph(String::new()));
            } else if tag_l.starts_with("img") {
                if let Some(src) = extract_attr(tag, "src") {
                    if let Ok(u) = base_url.join(&src) {
                        let key = u.to_string();
                        let alt = extract_attr(tag, "alt").unwrap_or_default();
                        page.blocks.push(Block::Image { key: key.clone(), alt });
                        images.push(ImageJob { key, url: u });
                    }
                }
            } else if tag_l.starts_with("a ") || tag_l == "a" {
                if let Some(href) = extract_attr(tag, "href") {
                    if let Some((inner, next)) = extract_until_close(&text, i + end + 1, "a") {
                        let u = base_url.join(&href).map(|x| x.to_string()).unwrap_or(href);
                        page.blocks.push(Block::Link { text: collapse_ws(&inner), href: u });
                        i = next;
                        continue;
                    }
                }
            }

            i = i + end + 1;
        } else {
            current_text.push(bytes[i] as char);
            i += 1;
        }
    }

    if !current_text.trim().is_empty() {
        page.blocks.push(Block::Paragraph(collapse_ws(&current_text)));
    }

    Ok((page, images))
}

fn decode_html_text(resp: &crate::net::http::HttpResponse) -> Result<String> {
    let content_type = resp
        .headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("Content-Type"))
        .map(|(_, v)| v.as_str())
        .unwrap_or("");

    // naive charset detection
    let charset = content_type
        .split(';')
        .find_map(|p| {
            let p = p.trim();
            p.strip_prefix("charset=")
                .or_else(|| p.strip_prefix("Charset="))
        })
        .map(|s| s.trim().trim_matches('"').to_ascii_lowercase());

    if let Some(cs) = charset {
        if cs.contains("gbk") || cs.contains("gb2312") || cs.contains("gb18030") {
            let (cow, _, _) = encoding_rs::GBK.decode(&resp.body);
            return Ok(cow.to_string());
        }
    }

    // default utf-8 with replacement
    Ok(String::from_utf8_lossy(&resp.body).to_string())
}

fn extract_tag_text(text: &str, tag: &str) -> Option<String> {
    let open = format!("<{}", tag);
    let close = format!("</{}>", tag);
    let start = text.to_ascii_lowercase().find(&open)?;
    let after_open = text[start..].find('>')? + start + 1;
    let end = text[after_open..].to_ascii_lowercase().find(&close)? + after_open;
    Some(collapse_ws(&text[after_open..end]))
}

fn extract_until_close(text: &str, from: usize, tag: &str) -> Option<(String, usize)> {
    let close = format!("</{}>", tag);
    let lower = text.to_ascii_lowercase();
    let end_rel = lower[from..].find(&close)?;
    let end = from + end_rel;
    let next = end + close.len();
    Some((strip_tags(&text[from..end]), next))
}

fn strip_tags(s: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for ch in s.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ => {
                if !in_tag {
                    out.push(ch);
                }
            }
        }
    }
    out
}

fn collapse_ws(s: &str) -> String {
    let mut out = String::new();
    let mut last_ws = false;
    for ch in s.chars() {
        if ch.is_whitespace() {
            if !last_ws {
                out.push(' ');
                last_ws = true;
            }
        } else {
            out.push(ch);
            last_ws = false;
        }
    }
    out.trim().to_string()
}

fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    // minimal attr parser, handles src="..." / src='...' / src=...
    let lower = tag.to_ascii_lowercase();
    let needle = format!("{}=", attr.to_ascii_lowercase());
    let idx = lower.find(&needle)?;
    let mut rest = tag[(idx + needle.len())..].trim_start();

    if rest.starts_with('"') {
        rest = &rest[1..];
        let end = rest.find('"')?;
        return Some(rest[..end].to_string());
    }
    if rest.starts_with('\'') {
        rest = &rest[1..];
        let end = rest.find('\'')?;
        return Some(rest[..end].to_string());
    }

    let end = rest
        .find(|c: char| c.is_whitespace() || c == '>')
        .unwrap_or(rest.len());
    Some(rest[..end].to_string())
}

#[allow(dead_code)]
fn _err(msg: impl Into<String>) -> Error {
    Error::Other(msg.into())
}
