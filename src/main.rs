use std::{env::set_current_dir, num::ParseFloatError};

use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Floor {
            target_ratio,
            current,
        } => calc_floor(target_ratio, current),
    }
}

fn calc_floor(target_ratio: f64, current: f64) {
    todo!()
}
