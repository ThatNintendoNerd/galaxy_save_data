use clap::{Parser, ValueEnum};

/// Convert Super Mario Galaxy save files to and from JSON.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The input save or JSON file path.
    pub input: String,

    /// The output save or JSON file path.
    pub output: Option<String>,

    /// The source or target console.
    #[arg(short, long, default_value_t, value_enum)]
    pub platform: Platform,

    /// Parse the save file even if the header is invalid.
    #[arg(short, long)]
    pub force: bool,
}

/// A compatible console.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum Platform {
    /// Nintendo Wii
    #[default]
    Wii,

    /// NVIDIA Shield TV
    ShieldTv,

    /// Nintendo Switch
    Switch,
}
