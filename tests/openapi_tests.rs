//! The committed `openapi.json` is the source for the TypeScript client in
//! `client/`, so it must match the spec generated from the handlers.
//!
//! Regenerate it with `UPDATE_OPENAPI=1 cargo test --test openapi_tests`.

#[test]
fn committed_openapi_spec_is_current() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/openapi.json");
    let generated = pakuvaraus::openapi().to_pretty_json().unwrap() + "\n";

    if std::env::var_os("UPDATE_OPENAPI").is_some() {
        std::fs::write(path, &generated).unwrap();
        return;
    }

    let committed = std::fs::read_to_string(path).unwrap_or_default();
    assert!(
        committed == generated,
        "openapi.json is stale; run `update-api-client` in the dev shell"
    );
}
