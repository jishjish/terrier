# Terrier

Unwieldy code base? Get to know your code better with Terrier.

## Quick Commands `cargo run`

- **Grep search for keywords**: `cargo run -- grep -p "src/main.rs" -k searching`
- **Get functions in file**: `cargo run -- func -p "src/main.rs"`
- **Find how functions are interconnected**: 
<!-- - **Function tree analysis**: `cargo run -- tree -p "src/main.rs"` -->

**Note**: After installation, replace `cargo run` with `terrier`

**Note**: Supported file types: `Rust`, `Python`.