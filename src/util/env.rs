//Parse bool from env var
pub fn parse_bool(val: &str) -> bool {
    matches!(val.trim().to_lowercase().as_str(), "true" | "yes" | "on" | "1")
}
