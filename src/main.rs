//cargo run "meow" 1> stdout.txt 2> stderr.txt
//cargo run "woof" 1> stdout.txt 2> stderr.txt
//cargo run

use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    /// documentation comment (triple slash)
    message: String, // [1]
    #[clap(default_value = "false", short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,
}

fn main() {
    let options = Options::parse();
    let message = options.message;
    let eye = if options.dead { "x" } else { "o" };
    println!("{}", message.bright_yellow().underline().on_purple());
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("     ( {eye} {eye} )", eye = eye.red().bold());
    println!("      =( I )=");

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark!")
    }
}
