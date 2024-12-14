use mcpack::elements;

#[test]
fn test_valid_element_type() {
    assert!(elements::is_valid_element_type("function")); // function with valid input
    assert!(!elements::is_valid_element_type("invalid")); // function with invalid input
}

#[test]
fn test_template_content() {
    let content = elements::get_sample_content("function");
    assert!(content.is_empty());

    let content = elements::get_sample_content("advancement");
    assert!(serde_json::from_str::<serde_json::Value>(&content).is_ok());
}
