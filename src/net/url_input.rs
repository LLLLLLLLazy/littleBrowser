use crate::net::{Error, Result};

pub fn parse_user_url(input: &str) -> Result<url::Url> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(Error::InvalidUrl("empty".into()));
    }

    // Assignment allows entering just host name (case-insensitive)
    let candidate = if trimmed.contains("://") {
        trimmed.to_string()
    } else {
        format!("http://{trimmed}/")
    };

    let mut u = url::Url::parse(&candidate).map_err(|e| Error::InvalidUrl(e.to_string()))?;

    if u.scheme().to_ascii_lowercase() != "http" {
        return Err(Error::UnsupportedScheme(u.scheme().to_string()));
    }

    // Normalize host to lower-case for hosts lookup
    if let Some(host) = u.host_str() {
        let lower = host.to_ascii_lowercase();
        // Rebuild URL with lower-case host to keep later joins consistent
        let mut rebuilt = u.clone();
        rebuilt.set_host(Some(&lower)).map_err(|e| Error::InvalidUrl(e.to_string()))?;
        u = rebuilt;
    }

    Ok(u)
}
