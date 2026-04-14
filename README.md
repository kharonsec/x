# x - Universal Project Runner

`x` is a zero-config, universal project runner. It automatically detects the type of project you're in and runs the appropriate build, test, or run commands.

## Installation
```sh
cargo build --release
```

## Usage
- `x` or `x run` - Runs the project (e.g., `cargo run`, `npm start`, `go run .`)
- `x build` - Builds the project (e.g., `cargo build`, `npm run build`)
- `x test` - Tests the project (e.g., `cargo test`, `npm test`, `pytest`)

## Supported Projects
- Rust (`Cargo.toml`)
- Node.js (`package.json`)
- Go (`go.mod`)
- Python (`pyproject.toml` or `requirements.txt`)
- Make (`Makefile`)
