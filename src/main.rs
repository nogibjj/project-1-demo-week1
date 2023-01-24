//A command-line tool to play Marco Polo game
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Noah Gift", about = "A Marco Polo game")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Play {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            if name == "Yayun" {
                println!("Hello! Yayun!");
            } else {
                println!("Hey! You are not Yayun!");
            }
        }
        None => println!("No subcommand was used"),
    }
}
