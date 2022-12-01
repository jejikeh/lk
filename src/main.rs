use anyhow::{Context, Result};
use clap::Parser;
use indicatif::ProgressBar;
use std::io::{self, Write};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn read_file_progress(n: u64, label: &str) {
    let pb = ProgressBar::new(n);

    let mut sum = 0;
    for i in 0..n {
        // Any quick computation, followed by an update to the progress bar.
        sum += 2 * i + 3;
        pb.inc(1);
    }
    pb.finish();

    println!("[{}] Sum ({}) calculated in {:?}", label, sum, pb.elapsed());
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut stdout = io::stdout().lock();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;
    writeln!(stdout, "{}", content)?;
    Ok(())
}
