# x - Universal Project Runner

`x` is a zero-config, universal project runner. It automatically detects the type of project you're in and executes the appropriate build, test, or run command for that ecosystem.

## Installation

### One-liner (requires Rust/Cargo)
```bash
curl -fsSL https://raw.githubusercontent.com/kharonsec/x/master/install.sh | bash
```

### Manual
```bash
git clone https://github.com/kharonsec/x.git
cd x
./install.sh
```

## Usage

- `x` or `x run`: Runs the project (e.g., `cargo run`, `npm start`, `go run .`)
- `x build`: Builds the project (e.g., `cargo build`, `npm run build`, `go build`)
- `x test`: Tests the project (e.g., `cargo test`, `npm test`, `go test ./...`)

## Supported Projects
- Rust (`Cargo.toml`)
- Node.js (`package.json`)
- Go (`go.mod`)
- Python (`pyproject.toml` or `requirements.txt`)
- Make (`Makefile`)
