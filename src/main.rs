use clap::{Args, Parser, Subcommand};

mod api;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "note-cli - a simple CLI to add, list and remove notes")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds a note
    Add(Add),

    /// Lists all notes
    List,
}

#[derive(Args)]
struct Add {
    /// the note as string
    note: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add(add)) => match add.note {
            Some(ref note) => {
                api::note::add(note);
            }
            None => {
                println!("Please provide a string to write the note")
            }
        },
        Some(Commands::List) => {
            api::note::list();
        }
        None => {}
    }
}
