extern crate clap;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, short)]
    newline: bool,
    #[arg(default_value = "")]
    whatever: String,
}

fn main() {
    let cli = Cli::parse();
    if cli.newline == true {
        print!("{}", cli.whatever)
    } else {
        println!("{}", cli.whatever)
    }
}
