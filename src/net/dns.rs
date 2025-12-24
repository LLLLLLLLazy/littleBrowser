use crate::net::{Error, Result};
use std::net::IpAddr;
use std::path::PathBuf;

pub fn resolve_via_hosts(host: &str) -> Result<IpAddr> {
    let host = host.trim().to_ascii_lowercase();
    let path = hosts_path();
    let content = std::fs::read_to_string(&path).map_err(|e| Error::HostsFileRead {
        path: path.display().to_string(),
        source: e,
    })?;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let ip_str = match parts.next() {
            Some(x) => x,
            None => continue,
        };
        let ip: IpAddr = match ip_str.parse() {
            Ok(x) => x,
            Err(_) => continue,
        };

        for name in parts {
            if name.to_ascii_lowercase() == host {
                return Ok(ip);
            }
        }
    }

    Err(Error::ResolveFailed {
        host,
        path: path.display().to_string(),
    })
}

fn hosts_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        // C:\Windows\System32\drivers\etc\hosts
        let root = std::env::var_os("SystemRoot").unwrap_or_else(|| "C:\\Windows".into());
        return PathBuf::from(root).join("System32").join("drivers").join("etc").join("hosts");
    }

    #[cfg(not(target_os = "windows"))]
    {
        return PathBuf::from("/etc/hosts");
    }
}
