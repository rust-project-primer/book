/// A simple key-value parser that parses strings of the form
/// `key=value\nkey=value\n...` into a list of key-value pairs.
///
/// This implementation has a subtle bug: it does not handle
/// values that contain the `=` character correctly.
// ANCHOR: parser
pub fn parse_config(input: &str) -> Vec<(&str, &str)> {
    let mut result = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            panic!("invalid config line: {}", line);
        }
        result.push((parts[0], parts[1]));
    }
    result
}
// ANCHOR_END: parser

// ANCHOR: parser_fixed
pub fn parse_config_fixed(input: &str) -> Result<Vec<(&str, &str)>, String> {
    let mut result = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("missing '=' in line: {line}"))?;
        if key.is_empty() {
            return Err(format!("empty key in line: {line}"));
        }
        result.push((key, value));
    }
    Ok(result)
}
// ANCHOR_END: parser_fixed
