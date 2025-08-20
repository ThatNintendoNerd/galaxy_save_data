use clap::Parser;

/// Convert Super Mario Galaxy 2 save files to and from JSON.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The input save or JSON file path.
    pub input: String,

    /// The output save or JSON file path.
    pub output: Option<String>,

    /// Parse the save file even if the header is invalid.
    #[arg(short, long)]
    pub force: bool,
}
