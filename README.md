# Terrier

Have you or someone you know been effected by vibe coding? A great idea that turned into an unweildy code base after one too many "make it better" prompts?
Well you are not entitled to compensation, but you mak get to know your project better with the help of Terrier.
Understand the structure, functions, and sources of your repo through our straightforward CLI.

**Note**: After installation, replace `cargo run` with `terrier`
**Note**: Supported file types: `Rust`, `Python`, `Javascript`.


## Quick Commands `cargo run`

- **Grep search for keywords**: `cargo run -- grep -p "src/main.rs" -k searching`
- **Get functions in file**: `cargo run -- func -p "src/main.rs"`
- **Find how functions are interconnected**: `cargo run -- link -p "src/"`
<!-- - **Function tree analysis**: `cargo run -- tree -p "src/main.rs"` -->



