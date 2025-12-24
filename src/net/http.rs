use crate::net::{dns, Error, Result};
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_code: u16,
    pub reason_phrase: String,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

pub fn build_assignment_like_get_request(url: &url::Url) -> Vec<u8> {
    let host = url.host_str().unwrap_or_default();
    let host_header = match url.port() {
        Some(p) if p != 80 => format!("{host}:{p}"),
        _ => host.to_string(),
    };
    let path = url.path();
    let mut path_and_query = path.to_string();
    if let Some(q) = url.query() {
        if !q.is_empty() {
            path_and_query.push('?');
            path_and_query.push_str(q);
        }
    }

    // Match the assignment's header set, using CRLF
    // Keep it close to the assignment's formatting.
    let req = format!(
        "GET {} HTTP/1.0\r\nHost:{}\r\nConnection:close\r\nUser-agent: WIN/4.0\r\nAccept: text/html,image/gif,image/jpeg\r\nAccept-language:cn\r\n\r\n",
        if path_and_query.is_empty() { "/" } else { &path_and_query },
        host_header
    );

    req.into_bytes()
}

pub fn fetch(url: &url::Url, request: &[u8]) -> Result<HttpResponse> {
    let host = url.host_str().ok_or_else(|| Error::InvalidUrl("missing host".into()))?;
    let port = url.port().unwrap_or(80);

    let ip = dns::resolve_via_hosts(host)?;
    let addr = SocketAddr::new(ip, port);

    let mut stream = TcpStream::connect(addr)?;
    stream.set_read_timeout(Some(Duration::from_secs(10)))?;
    stream.set_write_timeout(Some(Duration::from_secs(10)))?;

    stream.write_all(request)?;

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf)?;

    parse_http_response(&buf)
}

pub fn fetch_url_bytes(url: &url::Url) -> Result<Vec<u8>> {
    let req = build_assignment_like_get_request(url);
    let resp = fetch(url, &req)?;
    Ok(resp.body)
}

fn parse_http_response(raw: &[u8]) -> Result<HttpResponse> {
    let sep = b"\r\n\r\n";
    let header_end = raw
        .windows(sep.len())
        .position(|w| w == sep)
        .ok_or_else(|| Error::HttpParse("missing header/body separator".into()))?;

    let head_bytes = &raw[..header_end];
    let body = raw[(header_end + sep.len())..].to_vec();

    let head = std::str::from_utf8(head_bytes)
        .map_err(|e| Error::HttpParse(format!("headers not utf8: {e}")))?;

    let mut lines = head.split("\r\n");
    let status_line = lines
        .next()
        .ok_or_else(|| Error::HttpParse("missing status line".into()))?;

    let mut status_parts = status_line.splitn(3, ' ');
    let _http_ver = status_parts
        .next()
        .ok_or_else(|| Error::HttpParse("bad status line".into()))?;
    let code_str = status_parts
        .next()
        .ok_or_else(|| Error::HttpParse("bad status line".into()))?;
    let reason = status_parts.next().unwrap_or("").trim().to_string();
    let status_code: u16 = code_str
        .parse()
        .map_err(|e| Error::HttpParse(format!("bad status code: {e}")))?;

    let mut headers = BTreeMap::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let (k, v) = line
            .split_once(':')
            .ok_or_else(|| Error::HttpParse(format!("bad header line: {line}")))?;
        headers.insert(k.trim().to_string(), v.trim().to_string());
    }

    Ok(HttpResponse {
        status_code,
        reason_phrase: reason,
        headers,
        body,
    })
}
