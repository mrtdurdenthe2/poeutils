use arboard::Clipboard;
use clap::{Parser, Subcommand};
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Best { target_ratio: f64, current: f64 },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Best {
            target_ratio,
            current,
        } => calc_best(target_ratio, current),
    }
}

fn calc_best(target_ratio: f64, current: f64) -> Result<(), Box<dyn Error>> {
    let floor = (current / target_ratio).floor();
    let calc = floor * target_ratio;

    let whole = calc as i64;
    println!("Best: {} Ratio: {}", whole, floor);

    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_text(whole.to_string())?;
    Ok(())
}
