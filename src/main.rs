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
    Floor { target_ratio: f64, current: f64 },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Floor {
            target_ratio,
            current,
        } => calc_floor(target_ratio, current),
    }
}

fn calc_floor(target_ratio: f64, current: f64) -> Result<(), Box<dyn Error>> {
    let calc = ((current / target_ratio).floor()) * target_ratio;

    let whole = calc as i64;
    println!("{}", whole);

    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_text(whole.to_string())?;
    Ok(())
}
