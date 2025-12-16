use percent_encoding::percent_decode;
use std::collections::HashMap;
use std::fs;
use std::net::IpAddr;
use std::sync::OnceLock;

pub fn decode_uri(encoded: &str) -> String {
    percent_decode(encoded.as_bytes())
        .decode_utf8()
        .unwrap_or_default()
        .to_string()
}

static HOSTS_MAP: OnceLock<HashMap<IpAddr, String>> = OnceLock::new();

/// 从 hosts 文件解析 IP 到域名的映射
fn parse_hosts_file() -> HashMap<IpAddr, String> {
    let hosts_path = r"C:\Windows\System32\drivers\etc\hosts";
    let mut map = HashMap::new();

    if let Ok(content) = fs::read_to_string(hosts_path) {
        for line in content.lines() {
            let line = line.trim();

            // 跳过空行和注释行
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // 解析格式: IP地址 域名1 域名2 ...
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(ip) = parts[0].parse::<IpAddr>() {
                    // 使用第一个域名
                    map.insert(ip, parts[1].to_string());
                }
            }
        }
    }

    map
}

/// 将 IP 地址解析为域名（如果在 hosts 文件中有记录）
pub fn resolve_ip_to_hostname(ip: IpAddr) -> String {
    let hosts = HOSTS_MAP.get_or_init(parse_hosts_file);

    if let Some(hostname) = hosts.get(&ip) {
        format!("{} ({})", hostname, ip)
    } else {
        ip.to_string()
    }
}
