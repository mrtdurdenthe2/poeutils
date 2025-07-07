use arboard::Clipboard;
use clap::{Parser, Subcommand};
use std::error::Error;

// cargo install --path . --force
// Run this when building again to not build mult bins

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Best { target_ratio: f64, current: f64 },
    Liquid { amount_held: i64, curren_cur: i64 },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Best {
            target_ratio,
            current,
        } => calc_best(target_ratio, current),
        Command::Liquid {
            amount_held,
            curren_cur,
        } => liquidate(amount_held, curren_cur),
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

fn liquidate(amount_held: i64, current_curr_rate: i64) -> Result<(), Box<dyn Error>> {
    let calc = amount_held * current_curr_rate;

    println!("Most amount: {} Current: {}", calc, amount_held);

    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_text(calc.to_string())?;
    Ok(())
}
