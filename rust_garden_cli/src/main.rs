use clap::{Parser, Subcommand, CommandFactory};
use clap::error::ErrorKind;
use directories::UserDirs;
use std::path::PathBuf;

/// A CLI for the growing and curation of a digital garden
///
/// Visit https://ezektec.com for more!
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short = 'p', long, env)]
    garden_path: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Write something in your garden
    ///
    /// This command will open your $EDITOR, and wait for you
    /// to write something, and then save the file to your garden
    Write {
        /// Optionally set a title for what you are going to write about
        #[clap(short, long)]
        title: Option<String>,
    },
}

/// Get the user's garden directory, which by default
/// is placed in their home directory
fn get_default_garden_dir() -> Option<PathBuf> {
    UserDirs::new().map(|dirs| dirs.home_dir().join("Downloads/02 Rust-project/rust_garden_cli/garden"))
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    // dbg!(&args);

    let Some(garden_path) = 
            // get the file path entered by user
            // or construct the path to 'garden folder' where the user didn't supply a value
            // if it does not exist, we branch to the 'else'
            args.garden_path.or_else(get_default_garden_dir) 
        else {
            // here we check to see if the user supplied the path to write file to
            // if now value was provided, then emit an error message and exit the application
            let mut cmd = Args::command();
            cmd.error(
                ErrorKind::ValueValidation,
                format!(
                    "garden directory not provided and home directory unavailable for default garden directory"
                ),
            ).exit()
    };

    if !garden_path.exists() {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!(
                "garden directory `{}` doesn't exist, or is inaccessible",
                garden_path.display()
            ),
        ).exit()

    };

    // dbg!(garden_path);
    match args.cmd {
        Commands::Write { title } => {
            // dbg!(title);
            rust_garden_cli::write(garden_path, title)
        },
    }
}
