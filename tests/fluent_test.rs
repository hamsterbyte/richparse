#[cfg(feature = "intl")]
#[test]
fn test_rich_string_to_fluent_value() {
    use richparse::RichString;
    use fluent_bundle::FluentValue;
    use std::borrow::Cow;

    let input = "<red>Hello</red>";
    let rs = RichString::parse(input);
    let expected_ansi = rs.to_string();

    // Convert to FluentValue
    let fv: FluentValue = rs.into();

    // Verify
    if let FluentValue::String(cow_str) = fv {
        assert_eq!(cow_str, Cow::Borrowed(expected_ansi.as_str()));
    } else {
        panic!("Converted FluentValue is not a String variant");
    }
}
