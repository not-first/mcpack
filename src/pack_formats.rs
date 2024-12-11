use lazy_static::lazy_static;
use std::collections::HashMap;

pub const PACK_FORMATS: &[u8] = &[48, 57, 61];

#[derive(Debug, Clone)]
pub struct PackFormatInfo {
    pub versions: Vec<&'static str>,
}

lazy_static! {
    pub static ref PACK_FORMAT_VERSIONS: HashMap<u8, PackFormatInfo> = {
        let mut m = HashMap::new();
        m.insert(
            48,
            PackFormatInfo {
                versions: vec!["1.21", "1.21.1"],
            },
        );
        m.insert(
            57,
            PackFormatInfo {
                versions: vec!["1.21.2", "1.21.3"],
            },
        );
        m.insert(
            61,
            PackFormatInfo {
                versions: vec!["1.21.4"],
            },
        );
        m
    };
}

pub fn get_version_info(format: u8) -> Option<PackFormatInfo> {
    PACK_FORMAT_VERSIONS.get(&format).cloned()
}

pub fn is_valid_format(format: u8) -> bool {
    PACK_FORMAT_VERSIONS.contains_key(&format)
}

pub fn get_valid_formats_string() -> String {
    PACK_FORMATS
        .iter()
        .map(|&f| {
            let info = get_version_info(f).unwrap();
            format!("{} ({})", f, info.versions.join(", "))
        })
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn get_formats_string() -> String {
    PACK_FORMATS
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

fn parse_version(version: &str) -> Vec<u32> {
    let mut parts: Vec<u32> = version.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    // Pad with .0 if needed (e.g. "1.21" -> "1.21.0")
    while parts.len() < 3 {
        parts.push(0);
    }
    parts
}

/// Gets all valid pack formats between min and max inclusive
pub fn get_formats_in_range(min: u8, max: u8) -> Vec<u8> {
    PACK_FORMATS
        .iter()
        .copied()
        .filter(|&f| f >= min && f <= max)
        .collect()
}

/// Gets all Minecraft versions supported by a sequence of pack formats
pub fn get_version_range(formats: &[u8]) -> Vec<&'static str> {
    let mut versions = Vec::new();
    for &format in formats {
        if let Some(info) = get_version_info(format) {
            versions.extend(info.versions.iter().copied());
        }
    }
    versions.sort_by(|&a, &b| {
        let a_parts = parse_version(a);
        let b_parts = parse_version(b);
        a_parts.cmp(&b_parts)
    });
    versions.dedup();
    versions
}

/// Formats a list of versions into ranges where possible
pub fn format_version_range(versions: &[&str]) -> String {
    if versions.is_empty() {
        return String::new();
    }

    let mut ranges = Vec::new();
    let mut range_start = versions[0];
    let mut prev = versions[0];

    for &version in versions.iter().skip(1) {
        let prev_parts = parse_version(prev);
        let curr_parts = parse_version(version);

        // Check if versions are consecutive
        let consecutive = prev_parts
            .iter()
            .zip(curr_parts.iter())
            .rev()
            .find(|(p, c)| p != c)
            .map_or(false, |(p, c)| c - p == 1);

        if !consecutive {
            if prev == range_start {
                ranges.push(range_start.to_string());
            } else {
                ranges.push(format!(
                    "{}-{}",
                    range_start.trim_end_matches(".0"),
                    prev.trim_end_matches(".0")
                ));
            }
            range_start = version;
        }
        prev = version;
    }

    // Handle the last range
    if prev == range_start {
        ranges.push(range_start.trim_end_matches(".0").to_string());
    } else {
        ranges.push(format!(
            "{}-{}",
            range_start.trim_end_matches(".0"),
            prev.trim_end_matches(".0")
        ));
    }

    ranges.join(", ")
}
