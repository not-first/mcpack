use lazy_static::lazy_static;
use std::collections::HashMap;

pub const PACK_FORMATS: &[u8] = &[48, 57, 61];

lazy_static! {
    pub static ref PACK_FORMAT_VERSIONS: HashMap<u8, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert(48, vec!["1.21", "1.21.1"]);
        m.insert(57, vec!["1.21.2", "1.21.3"]);
        m.insert(61, vec!["1.21.4"]);
        m
    };
}

// get minecraft version(s)given a pack format
pub fn get_version_info(format: u8) -> Option<&'static Vec<&'static str>> {
    PACK_FORMAT_VERSIONS.get(&format)
}

// check if a given pack format is valid
pub fn is_valid_format(format: u8) -> bool {
    PACK_FORMAT_VERSIONS.contains_key(&format)
}

// get a formatted string of all valid pack formats
pub fn get_formats_string() -> String {
    PACK_FORMATS
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn parse_version(version: &str) -> Vec<u32> {
    let mut parts: Vec<u32> = version.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    // add .0 to the end if needed to help sorting (e.g. "1.21" -> "1.21.0")
    while parts.len() < 3 {
        parts.push(0);
    }
    parts
}

/// gets all valid pack formats between min and max inclusive format
pub fn get_formats_in_range(min: u8, max: u8) -> Vec<u8> {
    PACK_FORMATS
        .iter()
        .copied()
        .filter(|&f| f >= min && f <= max)
        .collect()
}

/// gets all minecraft versions supported by a sequence of pack formats
pub fn get_format_versions(formats: &[u8]) -> Vec<&'static str> {
    let mut versions = Vec::new();
    for &format in formats {
        if let Some(info) = get_version_info(format) {
            versions.extend(info.iter().copied());
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

/// formats a list of versions into ranges where possible
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

        // check if versions are consecutive (out of the supported formats)
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

    // handle the last range
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
