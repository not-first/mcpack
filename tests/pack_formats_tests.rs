use mcpack::pack_formats;

#[test]
fn test_version_info() {
    let info = pack_formats::get_version_info(48).unwrap();
    assert_eq!(*info, vec!["1.21", "1.21.1"]); // function with valid input

    let invalid_info = pack_formats::get_version_info(0);
    assert!(invalid_info.is_none()); // function with invalid input
}

#[test]
fn test_valid_pack_format() {
    assert!(pack_formats::is_valid_format(48)); // function with valid input
    assert!(!pack_formats::is_valid_format(0)); //function with invalid input
}

#[test]
fn test_version_parsing() {
    let version = "1.21";
    let parts = pack_formats::parse_version(version);
    assert_eq!(parts, vec![1, 21, 0]);

    let version = "1.21.1";
    let parts = pack_formats::parse_version(version);
    assert_eq!(parts, vec![1, 21, 1]);
}

#[test]
fn test_formats_in_range() {
    let formats = pack_formats::get_formats_in_range(48, 61);
    assert_eq!(formats, vec![48, 57, 61]);
}

#[test]
fn test_version_range() {
    let formats = [48, 61];
    let range = pack_formats::get_format_versions(&formats);
    assert_eq!(range, ["1.21", "1.21.1", "1.21.4"]);
}

#[test]
fn test_format_version_range() {
    let versions = vec!["1.21", "1.21.1", "1.21.2"];
    let range = pack_formats::format_version_range(&versions);
    assert_eq!(range, "1.21-1.21.2");

    let versions = vec!["1.21", "1.21.1", "1.21.2", "1.21.4"];
    let range = pack_formats::format_version_range(&versions);
    assert_eq!(range, "1.21-1.21.2, 1.21.4");
}
