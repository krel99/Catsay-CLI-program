//cargo run "meow" 1> stdout.txt 2> stderr.txt
//cargo run "woof" 1> stdout.txt 2> stderr.txt
//cargo run
//NO_COLOR=1 cargo run
//echo -n "blablabla" | catsay --stdin

use clap::Parser;
use colored::Colorize;
use std::io::{self, Read};

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    /// documentation comment (triple slash)
    message: String, // [1]
    #[clap(default_value = "false", short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,
    #[clap(short = 'i', long = "stdin")]
    /// Pipe stdin to catsay program as an argument
    stdin: bool,
}

fn main() -> Result<(), std::io::Error> {
    let options = Options::parse();
    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }
    let eye = if options.dead { "x" } else { "o" };

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark!")
    }

    println!("{}", message.bright_yellow().underline().on_purple());
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("     ( {eye} {eye} )", eye = eye.red().bold());
    println!("      =( I )=");

    Ok(())
}
