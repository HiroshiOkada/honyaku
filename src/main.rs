use std::io::{self, BufReader, Read};

use anyhow::{Context, Result};
use clap::Parser;

use honyaku::cli::Args;
use honyaku::env;
use honyaku::input::decode_stdin;
use honyaku::repl::run_repl;
use honyaku::translate::translate;

fn main() -> Result<()> {
    let args = Args::parse();
    let config = env::load(args.env.as_deref()).context("failed to load configuration")?;
    let runtime = tokio::runtime::Runtime::new().context("failed to start Tokio runtime")?;

    if args.repl {
        if !args.text.is_empty() {
            anyhow::bail!("--repl cannot be used with text arguments");
        }

        let stdin = io::stdin();
        let stdout = io::stdout();
        return runtime.block_on(run_repl(
            &config,
            BufReader::new(stdin.lock()),
            stdout.lock(),
            args.stdin_encoding,
            args.direction(),
        ));
    }

    let text = collect_input(&args).context("failed to read input")?;

    if text.trim().is_empty() {
        anyhow::bail!("no input text provided");
    }

    let translated = runtime.block_on(translate(&config, &text, args.direction()))?;

    println!("{}", translated);
    Ok(())
}

fn collect_input(args: &Args) -> Result<String> {
    if !args.text.is_empty() {
        Ok(args.text.join(" "))
    } else {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;
        decode_stdin(&buffer, args.stdin_encoding)
    }
}
