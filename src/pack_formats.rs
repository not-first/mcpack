pub struct SupportedVersion {
    pub format: [u32; 2],
    pub label: &'static str,
}

pub const SUPPORTED_VERSIONS: &[SupportedVersion] = &[
    SupportedVersion {
        format: [101, 1],
        label: "26.1 - 26.1.2",
    },
    SupportedVersion {
        format: [107, 1],
        label: "26.2",
    },
];

pub fn format_to_string(format: [u32; 2]) -> String {
    format!("{}.{}", format[0], format[1])
}

pub fn parse_format_string(s: &str) -> Option<[u32; 2]> {
    let mut parts = s.split('.');
    let major = parts.next()?.parse::<u32>().ok()?;
    let minor = parts.next()?.parse::<u32>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some([major, minor])
}

pub fn version_for_format(format: [u32; 2]) -> Option<&'static SupportedVersion> {
    SUPPORTED_VERSIONS.iter().find(|v| v.format == format)
}

pub fn is_supported_format(format: [u32; 2]) -> bool {
    version_for_format(format).is_some()
}

pub fn index_of_format(format: [u32; 2]) -> Option<usize> {
    SUPPORTED_VERSIONS.iter().position(|v| v.format == format)
}

pub fn cmp_format(a: [u32; 2], b: [u32; 2]) -> std::cmp::Ordering {
    a.cmp(&b)
}

pub fn version_label_range(min: [u32; 2], max: [u32; 2]) -> String {
    let min_label = version_for_format(min).map(|v| v.label).unwrap_or("unknown");
    let max_label = version_for_format(max).map(|v| v.label).unwrap_or("unknown");
    if min == max {
        min_label.to_string()
    } else {
        format!("{min_label} - {max_label}")
    }
}

pub fn format_from_json(value: &serde_json::Value) -> Option<[u32; 2]> {
    let arr = value.as_array()?;
    if arr.len() != 2 {
        return None;
    }
    Some([arr[0].as_u64()? as u32, arr[1].as_u64()? as u32])
}