//cargo run "meow" 1> stdout.txt 2> stderr.txt
//cargo run "woof" 1> stdout.txt 2> stderr.txt

use clap::Parser;

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
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("     ( {eye} {eye} )");
    println!("      =( I )=");

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark!")
    }
}
