//! Prints the `OpenAPI` description of the public API. The Nix build uses it
//! to generate the TypeScript client (`nix build .#api-client`).

fn main() -> serde_json::Result<()> {
    println!("{}", pakuvaraus::openapi().to_pretty_json()?);
    Ok(())
}
