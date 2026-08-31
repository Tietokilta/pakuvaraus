//Parse bool from env var
pub fn parse_bool(val: &str) -> bool {
    return match val.trim().to_lowercase().as_str() {
        "true" | "yes" | "on" | "1" => true,
        _ => false,
    };
}