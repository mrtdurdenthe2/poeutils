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
    let calc = ((current / target_ratio).floor()) * target_ratio;

    if target_ratio.fract() != 0.0 {
        let whole = calc as i64;
        println!("{}", whole)
    } else {
        println!("{}", calc)
    }
}
