// ANCHOR: config_type
pub struct Config {
    pub name: String,
    pub value: String,
}
// ANCHOR_END: config_type

// ANCHOR: parse_json
#[cfg(feature = "json")]
pub fn parse_json(input: &str) -> Config {
    // Minimal JSON-like parser for demonstration purposes.
    let get = |key: &str| -> &str {
        let pattern = format!("\"{key}\":\"");
        let start = input.find(&pattern).map(|i| i + pattern.len());
        start
            .map(|s| &input[s..input[s..].find('"').map(|e| s + e).unwrap_or(s)])
            .unwrap_or("unknown")
    };
    Config {
        name: get("name").to_string(),
        value: get("value").to_string(),
    }
}
// ANCHOR_END: parse_json

// ANCHOR: parse_yaml
#[cfg(feature = "yaml")]
pub fn parse_yaml(input: &str) -> Config {
    let mut name = "unknown";
    let mut value = "unknown";
    for line in input.lines() {
        if let Some(n) = line.strip_prefix("name: ") {
            name = n.trim();
        }
        if let Some(v) = line.strip_prefix("value: ") {
            value = v.trim();
        }
    }
    Config {
        name: name.to_string(),
        value: value.to_string(),
    }
}
// ANCHOR_END: parse_yaml

// ANCHOR: cfg_bug
#[cfg(any(feature = "json", feature = "yaml"))]
pub fn parse_auto(input: &str) -> Config {
    #[cfg(feature = "json")]
    if input.trim_start().starts_with('{') {
        return parse_json(input);
    }

    // Bug: this branch only compiles when "yaml" is enabled,
    // but the function is available with only "json" enabled.
    // With only "json", this function compiles but always panics.
    #[cfg(feature = "yaml")]
    {
        return parse_yaml(input);
    }

    #[cfg(not(feature = "yaml"))]
    panic!("no parser available for this format");
}
// ANCHOR_END: cfg_bug

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "json")]
    #[test]
    fn test_parse_json() {
        let input = r#"{"name":"test","value":"hello"}"#;
        let config = parse_json(input);
        assert_eq!(config.name, "test");
        assert_eq!(config.value, "hello");
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn test_parse_yaml() {
        let input = "name: test\nvalue: hello";
        let config = parse_yaml(input);
        assert_eq!(config.name, "test");
        assert_eq!(config.value, "hello");
    }

    // ANCHOR: test_parse_auto
    #[cfg(all(feature = "json", feature = "yaml"))]
    #[test]
    fn test_parse_auto() {
        let json = r#"{"name":"test","value":"hello"}"#;
        let config = parse_auto(json);
        assert_eq!(config.name, "test");

        let yaml = "name: test\nvalue: hello";
        let config = parse_auto(yaml);
        assert_eq!(config.name, "test");
    }
    // ANCHOR_END: test_parse_auto
}
