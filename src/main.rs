use clap::{Parser, Subcommand};
use std::process::Command;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about = "Universal project runner", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the project
    Build,
    /// Test the project
    Test,
    /// Run the project (default)
    Run,
}

enum ProjectType {
    Rust,
    Node,
    Go,
    Python,
    Make,
    Unknown,
}

fn detect_project() -> ProjectType {
    if Path::new("Cargo.toml").exists() {
        ProjectType::Rust
    } else if Path::new("package.json").exists() {
        ProjectType::Node
    } else if Path::new("go.mod").exists() {
        ProjectType::Go
    } else if Path::new("pyproject.toml").exists() || Path::new("requirements.txt").exists() {
        ProjectType::Python
    } else if Path::new("Makefile").exists() {
        ProjectType::Make
    } else {
        ProjectType::Unknown
    }
}

fn main() {
    let cli = Cli::parse();

    let project_type = detect_project();

    let (program, args) = match cli.command.unwrap_or(Commands::Run) {
        Commands::Build => match project_type {
            ProjectType::Rust => ("cargo", vec!["build"]),
            ProjectType::Node => ("npm", vec!["run", "build"]),
            ProjectType::Go => ("go", vec!["build"]),
            ProjectType::Python => {
                eprintln!("Python usually doesn't need a build step, or depends on specific tools.");
                std::process::exit(1);
            }
            ProjectType::Make => ("make", vec!["build"]),
            ProjectType::Unknown => {
                eprintln!("Unknown project type. Cannot build.");
                std::process::exit(1);
            }
        },
        Commands::Test => match project_type {
            ProjectType::Rust => ("cargo", vec!["test"]),
            ProjectType::Node => ("npm", vec!["test"]),
            ProjectType::Go => ("go", vec!["test", "./..."]),
            ProjectType::Python => ("pytest", vec![]),
            ProjectType::Make => ("make", vec!["test"]),
            ProjectType::Unknown => {
                eprintln!("Unknown project type. Cannot test.");
                std::process::exit(1);
            }
        },
        Commands::Run => match project_type {
            ProjectType::Rust => ("cargo", vec!["run"]),
            ProjectType::Node => ("npm", vec!["start"]),
            ProjectType::Go => ("go", vec!["run", "."]),
            ProjectType::Python => ("python", vec!["main.py"]),
            ProjectType::Make => ("make", vec!["run"]),
            ProjectType::Unknown => {
                eprintln!("Unknown project type. Cannot run.");
                std::process::exit(1);
            }
        },
    };

    println!("Running: {} {}", program, args.join(" "));

    let status = Command::new(program)
        .args(&args)
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
