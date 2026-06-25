use std::io::{self, Read};

use anyhow::{Context, Result};
use clap::Parser;

use honyaku::cli::Args;
use honyaku::env;
use honyaku::translate::translate;

fn main() -> Result<()> {
    let args = Args::parse();
    let config = env::load(args.env.as_deref()).context("failed to load configuration")?;
    let text = collect_input(&args).context("failed to read input")?;

    if text.trim().is_empty() {
        anyhow::bail!("no input text provided");
    }

    let runtime = tokio::runtime::Runtime::new().context("failed to start Tokio runtime")?;
    let translated = runtime.block_on(translate(&config, &text, args.direction()))?;

    println!("{}", translated);
    Ok(())
}

fn collect_input(args: &Args) -> Result<String> {
    if !args.text.is_empty() {
        Ok(args.text.join(" "))
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}
