use anyhow::{Context, Ok, Result};
use clap::Parser;
use std::io::{self, Write};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn find_matches(content: &str, pattern: &str) -> String {
    let mut i = 0;
    let mut result = String::new();
    for line in content.lines() {
        i += 1;
        if line.contains(pattern) {
            result.push_str(&(line.to_owned() + " : " + &i.to_string().as_str().to_owned() + "\n"));
        }
    }

    result
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut stdout = io::stdout().lock();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;

    if &args.pattern == "*" {
        writeln!(stdout, "{}", &content)?;
        return Ok(());
    }
    writeln!(stdout, "{}", find_matches(&content, &args.pattern))?;

    Ok(())
}
