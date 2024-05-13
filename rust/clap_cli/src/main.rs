use clap::Parser;
// anyhow error handling
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

// Run the command to test:
// cargo run -- some-pattern some-file
// test on the same file that created a program cargo run -- main src/main.rs
// match main
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let result = std::fs::read_to_string("test.txt");
//     let content = match result {
//         Ok(content) => { content },
//         Err(error) => { return Err(error.into()); }
//     };
//     println!("file content: {}", content);
//     Ok(())
// }

//anyhow main
fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
