mod commands;
use commands::init::init;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    println!("Hello, world!");
    print_name();
    init();

    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}

fn print_name() {
    let name: &str = "Rizwan";
    println!("my name is !{}", name);
}
