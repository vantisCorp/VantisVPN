//! # Utility Functions
//!
//! Common utility functions used throughout VANTISVPN.

use std::time::{SystemTime, UNIX_EPOCH};

/// Get current timestamp in seconds
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Get current timestamp in milliseconds
pub fn current_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

/// Format bytes to human-readable size
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

    if bytes == 0 {
        return "0 B".to_string();
    }

    let bytes = bytes as f64;
    let unit_index = (bytes.log2() / 10.0).floor() as usize;
    let unit_index = unit_index.min(UNITS.len() - 1);
    let size = bytes / 1024_f64.powi(unit_index as i32);

    format!("{:.2} {}", size, UNITS[unit_index])
}

/// Format seconds to human-readable duration
pub fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}

/// Check if string is valid IPv4 address
pub fn is_valid_ipv4(s: &str) -> bool {
    s.parse::<std::net::Ipv4Addr>().is_ok()
}

/// Check if string is valid IPv6 address
pub fn is_valid_ipv6(s: &str) -> bool {
    s.parse::<std::net::Ipv6Addr>().is_ok()
}

/// Generate random string of specified length
pub fn random_string(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";

    (0..length)
        .map(|_| {
            let idx = (rand::random::<u32>() as usize) % CHARSET.len();
            CHARSET[idx] as char
        })
        .collect()
}

/// Sleep for specified duration (async)
pub async fn sleep_secs(seconds: u64) {
    tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
}

/// Sleep for specified milliseconds (async)
pub async fn sleep_ms(ms: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0), "0s");
        assert_eq!(format_duration(30), "30s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3661), "1h 1m 1s");
    }

    #[test]
    fn test_is_valid_ipv4() {
        assert!(is_valid_ipv4("192.168.1.1"));
        assert!(is_valid_ipv4("0.0.0.0"));
        assert!(!is_valid_ipv4("256.0.0.0"));
        assert!(!is_valid_ipv4("not.an.ip"));
    }

    #[test]
    fn test_is_valid_ipv6() {
        assert!(is_valid_ipv6("::1"));
        assert!(is_valid_ipv6("2001:db8::1"));
        assert!(!is_valid_ipv6("not.an.ipv6"));
    }

    #[test]
    fn test_random_string() {
        let s1 = random_string(16);
        let s2 = random_string(16);

        assert_eq!(s1.len(), 16);
        assert_eq!(s2.len(), 16);
        assert_ne!(s1, s2);
    }
}
